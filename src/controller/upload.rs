use crate::configuration::AppState;
use crate::model::usermodel::ConversionError;
use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use crate::model::{DeleteError, UploadResponse};
use crate::service::delete_file_service;

pub async fn upload_file(
    State(app_state): State<AppState>,
    mut file: Multipart,
) -> Result<UploadResponse, ConversionError> {
    let mut links = UploadResponse::new("".to_string(), "".to_string());

    tracing::info!("Received request to upload file(s)");

    while let Some(field) = file.next_field().await? {
        // logic to handle upload, probably calling a service
        links = crate::service::upload_file_data(
            field,
            app_state.clone(),
            crate::model::UploadOptions {
                aws_upload: Some(true),
            },
        )
        .await?;
    }
    Ok(links)
}

pub async fn delete_file(
    State(app_state): State<AppState>,
    Path(delete_token): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {

    match delete_file_service(&app_state.pg_pool, delete_token.as_str(), app_state.settings.application.aws_settings.bucket_name.as_str(), app_state.sender).await {
        Ok(response) => Ok(response.into_response()),
        Err(DeleteError::NotFound(StatusCode::NOT_FOUND)) => Ok((StatusCode::NOT_FOUND, "File not found").into_response()),
        _ => Ok((StatusCode::INTERNAL_SERVER_ERROR, "Error Deleting File").into_response())
    }
}
