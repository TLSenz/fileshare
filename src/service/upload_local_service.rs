use crate::controller::create_link;
use crate::model::ConversionError::ConversionError as OtherConversionError;
use crate::model::{ConversionError, FileToInsert, UploadOptions};
use crate::repository::check_if_file_name_exists;
use crate::service::upload_aws;
use axum::extract::Multipart;
use bcrypt::hash;
use bytes::Bytes;
use sqlx::PgPool;
use std::fs::File;
use std::io::Write;
use crate::configuration::AppState;

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
    mut file_data: Multipart,
    app_state: AppState,
    upload_options: UploadOptions,
) -> Result<(), ConversionError> {
    let mut links = String::new();
    while let Some(field) = file_data.next_field().await? {
        let mut content_type = String::new();
        tracing::info!("GOt into while Loop");

        let other_file_name = field
            .name()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unnamed".to_string());

        tracing::info!(original_name = %other_file_name, "Processing multipart field");

        let _exists = check_if_file_name_exists(&app_state.pg_pool, other_file_name.clone()).await?;
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
        tracing::info!(original_name = %other_file_name, bytes = size, "Calculated hashes");
        let mut file_struct: FileToInsert = FileToInsert {
            file_name: other_file_name.clone(),
            hashed_file_name: name_link_hash.clone(),
            content_hash: data_hash.clone(),
            content_type: content_type.clone(),
            size,
            storage_path: filename.clone(),
            owner_id: None,
            is_public: Some(1),
            is_deleted: Some(0),
        };

        match upload_options.aws_upload {
            Some(aws) => {
                if aws {
                    upload_aws(&app_state, &file_struct, &data).await?;
                }
            }
            None => {
                //Implement Check what the default Upload PLace for a File is, either Local or S*
                write_data(&data, &filename).await?;
            }
        }
        // aws(&data, &file_struct).await?;

        tracing::info!(original_name = %other_file_name, %filename, "Stored file, creating download link");
        let other_link = create_link(&app_state.pg_pool, file_struct).await?;
        tracing::info!(link = %other_link, "Created download link");
        links = other_link
    }
    Ok(())
}
