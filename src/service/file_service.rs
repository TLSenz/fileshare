use crate::configuration::AppState;
use crate::model::ConversionError::ConversionError as OtherConversionError;
use crate::model::{ConversionError, DeleteError, DeleteWorkerRequest, FileToInsert, UploadOptions, UploadResponse};
use crate::repository::{check_if_file_name_available, set_file_deleted, validate_delete_token, write_file_info_to_db};
use crate::service::upload_aws;
use bcrypt::hash;
use bytes::Bytes;
use sqlx::PgPool;
use std::fs::File;
use std::io::Write;
use axum::http::StatusCode;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

#[tracing::instrument(skip(data))]
pub async fn write_data(data: &Bytes, filepath: &String) -> Result<(), ConversionError> {
    tracing::info!(path = %filepath, bytes = data.len(), "Writing data to local storage");
    let mut file = File::create(filepath)
        .map_err(|_| OtherConversionError("Error Creating File".to_string()))?;
    tracing::info!("Created File");
    file.write(data)
        .map_err(|_| OtherConversionError("Error writing Data to File".to_string()))?;
    tracing::info!("Wrote to file");
    Ok(())
}

pub async fn upload_file_data(
    file_data: axum::extract::multipart::Field<'_>,
    app_state: AppState,
    upload_options: UploadOptions,
) -> Result<UploadResponse, ConversionError> {
    let mut links = String::new();
    // while let Some(field) = file_data.next_field().await? {
    let field = file_data;
    let mut content_type = String::new();
    tracing::info!("GOt into while Loop");

    let other_file_name = field
        .file_name()
        .map(|s| s.to_string())
        .unwrap_or_else(|| "unnamed".to_string());

    tracing::info!(original_name = %other_file_name, "Processing multipart field");

    let _exists = check_if_file_name_available(&app_state.pg_pool, other_file_name.clone()).await?;
    tracing::info!("got datavbase call");
    let file_type = field.content_type();

    tracing::info!("matching file type");
    match file_type {
        Some(file_type) => {
            let filetype_splited: Vec<&str> = file_type.split('/').collect();
            content_type = filetype_splited
                .get(1)
                .unwrap_or(&"octet-stream")
                .to_string();
        }
        None => {
            content_type = "txt".to_string();
        }
    }

    let filename = format!("content/{}.{}", other_file_name, content_type);
    let data = field.bytes().await.map_err(ConversionError::from)?;

    tracing::info!(%filename, bytes = data.len(), "Received file data");

    let size = data.len();
    let size = size.try_into()?;

    tracing::info!(original_name = %other_file_name, bytes = size, "Calculating hashes");
    let name_link_hash = hash(filename.clone(), 4)?;
    let data_hash = hash(data.clone(), 4)?;
    let delete_token = Uuid::new_v4().to_string();
    tracing::info!(original_name = %other_file_name, bytes = size, "Calculated hashes");
    let file_struct: FileToInsert = FileToInsert {
        file_name: other_file_name.clone(),
        hashed_file_name: name_link_hash.clone(),
        content_hash: data_hash.clone(),
        content_type: content_type.clone(),
        delete_token: delete_token.clone(),
        size,
        storage_path: filename.clone(),
        owner_id: None,
        is_public: false,
        is_deleted: false,
        on_aws: true,
    };

    match upload_options.aws_upload {
        Some(aws) => {
            if aws {
                upload_aws(&app_state, &file_struct, &data).await?;
            }
        }
        None => {
            tracing::error!("writing File to ");
            //Implement Check what the default Upload PLace for a File is, either Local or S*
            write_data(&data, &filename).await?;
        }
    }
    // aws(&data, &file_struct).await?;

    tracing::info!(original_name = %other_file_name, %filename, "Stored file, creating download link");
    let other_link = create_link(&app_state.pg_pool, &file_struct).await?;
    tracing::info!(link = %other_link, "Created download link");
    let upload_response = UploadResponse::new(other_link, delete_token);
    Ok(upload_response)
}

#[tracing::instrument(skip(pool))]
pub async fn create_link(pool: &PgPool, file: &FileToInsert) -> Result<String, ConversionError> {
    tracing::debug!(?file, "Writing file metadata to DB");
    let file = write_file_info_to_db(pool, file)
        .await
        .map_err(|_| OtherConversionError("Error writing to database".to_string()))?;

    tracing::debug!(hashed_name = %file.hashed_file_name, "Generating public link");
    let other_link = format!(
        "http://127.0.0.1:3000/api/download/{}",
        urlencoding::encode(&file.hashed_file_name)
    );
    Ok(other_link)
}

    pub async fn delete_file_service(pool: &PgPool, delete_token: &str, bucket_name: &str,  tx: Sender<DeleteWorkerRequest>) -> Result<String, DeleteError> {
    let does_file_exist = validate_delete_token(pool, delete_token).await;
        match does_file_exist {
            Ok(true) => {
                let file_key = set_file_deleted(pool, delete_token).await.map_err(|_| DeleteError::DeletionFailed(StatusCode::NOT_FOUND))?;
                let request = DeleteWorkerRequest::new(delete_token.to_string(), bucket_name.to_string(), file_key.to_string());
                tx.send(request).await.ok();
                Ok(format!("Successfully deleted file {}", file_key))
            },
            Ok(false) => {
                return Err(DeleteError::NotFound(StatusCode::NOT_FOUND));
            },
            Err(_) => {
                return Err(DeleteError::DeletionFailed(StatusCode::INTERNAL_SERVER_ERROR));
            }
        }
}




