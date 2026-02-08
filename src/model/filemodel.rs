use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct GetFileResponse {
    pub(crate) filename: String,
    pub(crate) filepath: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UploadResponse {
    pub link: String,
    pub delete_token: String,
}

impl UploadResponse {
    pub fn new(link: String, delete_token: String) -> Self {
        Self { link, delete_token }
    }
}

impl IntoResponse for UploadResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Serialize, Deserialize)]
pub struct UploadOptions {
    pub aws_upload: Option<bool>,
}

pub enum DeleteError {
    NotFound(StatusCode),
    DeletionFailed(StatusCode),
    AwsError(StatusCode),
}

pub struct DeleteWorkerRequest {
    pub uuid: Uuid,
    pub delete_token: String,
    pub file_key: String,
    pub bucket_name: String,
}

impl DeleteWorkerRequest {
    pub fn new(delete_token: String, bucket_name: String, file_key: String) -> Self {
        let uuid = Uuid::new_v4();
        Self {uuid, delete_token, bucket_name, file_key}
    }
}
