use crate::model::usermodel;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use redis::RedisError;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;

#[derive(Deserialize, Serialize, Clone)]
pub struct EncodeJWT {
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) exp: usize, // Expiration time (as UTC timestamp)
}

#[derive(Debug)]
pub enum AuthError {
    AuthError(String, StatusCode),
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::AuthError(message, status) => {
                write!(f, "Error: {},StatusCode: {}", message, status)
            }
        }
    }
}

impl std::error::Error for AuthError {}

impl From<usermodel::ConversionError> for AuthError {
    fn from(err: usermodel::ConversionError) -> Self {
        match err {
            usermodel::ConversionError::ConversionError(msg) => AuthError::AuthError(
                format!("Authentication error: {}", msg),
                StatusCode::UNAUTHORIZED,
            ),
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            AuthError::AuthError(message, status) => {
                let body = serde_json::json!({
                    "error": message
                });
                (status, axum::Json(body)).into_response()
            }
        }
    }
}
#[derive(Debug)]
pub enum RateError<'a> {
    RateError(&'a str, StatusCode),
}
impl From<redis::RedisError> for RateError<'_> {
    fn from(value: RedisError) -> Self {
        match value {
            _redis_error => RateError::RateError(
                "Could not get value from Redis ",
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        }
    }
}
impl fmt::Display for RateError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RateError::RateError(message, status) => {
                write!(f, "Error: {},StatusCode: {}", message, status)
            }
        }
    }
}
impl IntoResponse for RateError<'_> {
    fn into_response(self) -> Response {
        match self {
            RateError::RateError(message, status_code) => {
                let body = serde_json::json!({
                    "Error Message": message
                });
                (status_code, axum::Json(body)).into_response()
            }
        }
    }
}
