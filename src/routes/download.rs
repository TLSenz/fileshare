use std::fmt::Error;
use axum::extract::{Path, State};
use axum::body::*;
use axum::response::IntoResponse;
use axum::http::{header, Response, StatusCode};
use sqlx::PgPool;
use crate::model::filemodel::GetFileResponse;
use crate::model::usermodel::ConversionError::*;
use crate::repository::filerepository::get_file_name_from_db;

pub async fn download(
    State(pool): State<PgPool>,
    Path(file_link): Path<String>
) -> impl IntoResponse {
    println!("Processing Request");

    let information = get_file_name(pool, file_link).await;

    match information {
        Ok(infos) => {
            let content_types = mime_guess::from_path(&infos.filepath);
            let file_data = tokio::fs::read(&infos.filepath).await;
            match file_data {
                Ok(data) => {
                    let body = Body::from(data);
                    
                    Response::builder()
                        .header(header::CONTENT_TYPE, content_types.first_raw().unwrap())
                        .body(body)
                        .unwrap()
                }
                Err(_) => {
                    println!("Error Reading Data");
                    (StatusCode::INTERNAL_SERVER_ERROR, "File not found").into_response()
                }
            }
        }
        Err(error) => {
            println!("Error message while try to get File Path: {}", error);
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        }
    }
}

pub async fn get_file_name(pool: PgPool, file_link: String) -> Result<GetFileResponse, Error> {
    let file_link: Vec<_> = file_link.split("/").collect();
    let file_name_hash = file_link[file_link.len() - 1];

    let file = get_file_name_from_db(pool, file_name_hash.to_string()).await?;

    let file_names = &file.file_name;
    let file_paths = &file.storage_path;

    let res: GetFileResponse = GetFileResponse {
        filename: file_names.to_string(),
        filepath: file_paths.to_string()
    };

    Ok(res)
}
