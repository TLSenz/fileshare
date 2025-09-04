use std::env::VarError;
use std::fmt;
use std::fmt::Formatter;
use std::io::Error;
use std::num::TryFromIntError;
use axum::extract::multipart::MultipartError;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use bcrypt::BcryptError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::task::JoinError;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub password: String,
    pub email: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    pub name: String,
    pub password: String,
    pub email: String
}

pub struct LoginResponse {
    pub status_code: StatusCode,
    pub jwt_token: String
}

impl IntoResponse for LoginResponse {
    fn into_response(self) -> Response {
        let res_json = serde_json::json!({
            "token" : self.jwt_token,
        });
        (StatusCode::OK, Json(res_json)).into_response()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, FromRow)]
pub struct File {
    pub id: Option<i32>,
    pub file_name: String,
    pub hashed_file_name: String,
    pub content_hash: String,
    pub content_type: String,
    pub size: i32,
    pub storage_path: String,
    pub owner_id: Option<i32>,
    pub is_public: Option<i32>,
    pub is_deleted: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FileToInsert {
    pub file_name: String,
    pub hashed_file_name: String,
    pub content_hash: String,
    pub content_type: String,
    pub size: i32,
    pub storage_path: String,
    pub owner_id: Option<i32>,
    pub is_public: Option<i32>,
    pub is_deleted: Option<i32>,
    // Timestamps are omitted here because the SQL schema has DEFAULT CURRENT_TIMESTAMP for them
}

#[derive(Debug)]
pub enum ConversionError {
    ConversionError(String)
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ConversionError::ConversionError(message) => write!(f,"Conversion Error {} ", message)
        }
    }
}

impl std::error::Error for ConversionError {
}

impl From<TryFromIntError> for ConversionError {
    fn from(value: TryFromIntError) -> Self {
        ConversionError::ConversionError(format!("Could nor convert: {} ", value))
    }
}

impl From<BcryptError> for ConversionError {
    fn from(value: BcryptError) -> Self {
        ConversionError::ConversionError(format!("Error Message:{}", value))
    }
}

impl IntoResponse for ConversionError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro with Storing File and Provide Link: {}", self)).into_response()
    }
}

impl From<MultipartError> for ConversionError {
    fn from(err: MultipartError) -> Self {
        ConversionError::ConversionError("Erorr".to_string())
    }
}

impl From<VarError> for ConversionError {
    fn from(value: VarError) -> Self {
        ConversionError::ConversionError("Error Converting stuff".to_string())
    }
}

impl From<JoinError> for ConversionError {
    fn from(value: JoinError) -> Self {
        println!("{}", value);
        ConversionError::ConversionError("Error Join Handle".to_string())
    }
}

impl From<Box<dyn std::error::Error>> for ConversionError {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        ConversionError::ConversionError("Error".to_string())
    }
}

impl From<sqlx::Error> for ConversionError {
    fn from(value: sqlx::Error) -> Self {
        ConversionError::ConversionError(format!("Database Error: {}", value))
    }
}
