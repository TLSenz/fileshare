use axum::extract::{Multipart, Path, State};
use axum::body::*;
use axum::response::IntoResponse;
use axum::http::{header, Response, StatusCode};
use sqlx::PgPool;
use crate::model::usermodel::ConversionError;
use crate::model::usermodel::ConversionError::*;
use crate::service::fileservice::{get_file_name, store_files};

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

pub async fn upload_file(
    State(pool): State<PgPool>,
    file: Multipart
) -> Result<String, ConversionError> {
    let is_stored = store_files(pool, file).await;
    match is_stored {
        Ok(links) => {
            if let Some(first_link) = links.into_iter().next() {
                Ok(first_link)
            } else {
                println!("No links returned from store_files");
                Err(ConversionError("No files were stored".to_string()))
            }
        }
        Err(error) => {
            println!("Error storing files: {}", error);
            Err(error)
        }
    }
}