use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct GetFileResponse {
    pub(crate) filename: String,
    pub(crate) filepath: String,
}

#[derive(Serialize, Deserialize)]
pub struct UploadOptions {
    pub aws_upload: Option<bool>,
}

pub enum DeleteError {
    NotFound(StatusCode),
    DeletionFailed(StatusCode),
}

pub struct DeleteWorkerRequest {
    pub uuid: Uuid,
    pub delete_token: String,
}
