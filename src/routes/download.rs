use std::fmt::Error;
use axum::extract::{Path, State};
use axum::body::*;
use axum::response::IntoResponse;
use axum::http::{header, Response, StatusCode};
use sqlx::PgPool;
use uuid::Uuid;
use crate::model::filemodel::GetFileResponse;
use crate::repository::filerepository::get_file_name_from_db;

#[tracing::instrument(skip(pool))]
pub async fn download(
    State(pool): State<PgPool>,
    Path(file_link): Path<String>
) -> impl IntoResponse {
    let request_id = Uuid::new_v4();
    tracing::info!(
        %request_id,
        file_link = %file_link,
        "Downloading file"
    );

    let information = get_file_name(pool, &file_link).await;

    match information {
        Ok(infos) => {
            let content_types = mime_guess::from_path(&infos.filepath);
            let file_data = tokio::fs::read(&infos.filepath).await;
            match file_data {
                Ok(data) => {
                    tracing::info!(%request_id, path = %infos.filepath, bytes = data.len(), "Sending file");
                    let body = Body::from(data);

                    Response::builder()
                        .header(header::CONTENT_TYPE, content_types.first_raw().unwrap_or("application/octet-stream"))
                        .body(body)
                        .unwrap()

                }
                Err(e) => {
                    tracing::error!(
                        %request_id,
                        path = %infos.filepath,
                        error = %e,
                        "Error reading file from disk"
                    );
                    (StatusCode::INTERNAL_SERVER_ERROR, "File not found").into_response()
                }
            }
        }
        Err(error) => {
            tracing::error!(
                %request_id,
                error = %error,
                "Error getting URL path for file link"
            );
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

#[tracing::instrument(skip(pool))]
pub async fn get_file_name(pool: PgPool, file_link: &str) -> Result<GetFileResponse, Error> {
    tracing::debug!(%file_link, "Querying DB for file by link");

    let file = get_file_name_from_db(pool, file_link).await?;

    let file_names = &file.file_name;
    let file_paths = &file.storage_path;

    let res: GetFileResponse = GetFileResponse {
        filename: file_names.to_string(),
        filepath: file_paths.to_string()
    };

    tracing::debug!(filename = %res.filename, filepath = %res.filepath, "Resolved file metadata");
    Ok(res)
}
