use crate::configuration::AppState;
use crate::model::usermodel::ConversionError::*;
use crate::model::usermodel::{ConversionError, FileToInsert};
use crate::repository::filerepository::{check_if_file_name_exists, write_name_to_db};
use crate::service::write_data;
use aws_sdk_s3::primitives::ByteStream;
use axum::extract::{Multipart, State};
use bcrypt::hash;
use bytes::Bytes;
use sqlx::PgPool;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

pub async fn upload_file(
    State(appState): State<AppState>,
    mut file: Multipart,
) -> Result<String, ConversionError> {
    let mut links = String::new();

    tracing::info!("Received request to upload file(s)");

    while let Some(field) = file.next_field().await? {
        let mut content_type = String::new();
        tracing::info!("GOt into while Loop");

        let other_file_name = field
            .name()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unnamed".to_string());

        tracing::info!(original_name = %other_file_name, "Processing multipart field");

        let _exists = check_if_file_name_exists(&appState.pg_pool, other_file_name.clone()).await?;
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
        let file_struct: FileToInsert = FileToInsert {
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

        // aws(&data, &file_struct).await?;
        write_data(&data, &filename).await?;

        tracing::info!(original_name = %other_file_name, %filename, "Stored file, creating download link");
        let other_link = create_link(&appState.pg_pool, file_struct).await?;
        tracing::info!(link = %other_link, "Created download link");
        links = other_link
    }
    Ok(links)
}

#[tracing::instrument(skip(pool))]
pub async fn create_link(pool: &PgPool, file: FileToInsert) -> Result<String, ConversionError> {
    tracing::debug!(?file, "Writing file metadata to DB");
    let file = write_name_to_db(pool, file)
        .await
        .map_err(|_| ConversionError("Error writing to database".to_string()))?;

    tracing::debug!(hashed_name = %file.hashed_file_name, "Generating public link");
    let other_link = format!(
        "http://127.0.0.1:3000/api/download/{}",
        urlencoding::encode(&file.hashed_file_name)
    );
    Ok(other_link)
}
