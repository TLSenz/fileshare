use crate::configuration::LogFormat::Json;
use crate::model::File;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct GetFileResponse {
    pub(crate) filename: String,
    pub(crate) filepath: String,
}

// Own the data to avoid lifetime/dangling reference issues
#[derive(Serialize, Deserialize)]
pub struct FileDTO {
    pub filename: String,
    pub size: i32,
    pub content_type: String,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
}

impl From<File> for FileDTO {
    fn from(value: File) -> Self {
        FileDTO {
            filename: value.file_name,
            size: value.size,
            content_type: value.content_type,
            is_public: true,
            created_at: value.created_at.unwrap_or(Utc::now()), // default to now if missing
        }
    }
}

impl IntoResponse for FileDTO {
    fn into_response(self) -> Response {
        (StatusCode::OK, self).into_response()
    }
}
