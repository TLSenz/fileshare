use crate::configuration::AppState;
use crate::model::usermodel::{ConversionError};
use axum::extract::{Multipart, State};
use crate::model::usermodel::FileToInsert;

pub async fn upload_file(
    State(appState): State<AppState>,
    mut file: Multipart,
) -> Result<String, ConversionError> {
    let mut links = String::new();

    tracing::info!("Received request to upload file(s)");

    while let Some(field) = file.next_field().await? {
       // logic to handle upload, probably calling a service
       links = crate::service::upload_file_data(field, appState.clone(), crate::model::UploadOptions { aws_upload: Some(true) }).await?;
    }
    Ok(links)
}

