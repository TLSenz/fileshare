use std::fmt;
use std::fmt::Formatter;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use crate::model::usermodel;


#[derive(Deserialize, Serialize)]
pub struct EncodeJWT{
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) exp: usize  // Expiration time (as UTC timestamp)
}

#[derive(Debug)]
pub enum AuthError {
    AuthError(String,StatusCode)
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::AuthError(message, status) => write!(f,"Error: {},StatusCode: {}", message, status)
        }

    }
}


impl std::error::Error for AuthError{
}

impl From<usermodel::ConversionError> for AuthError {
    fn from(err: usermodel::ConversionError) -> Self {
        match err {
            usermodel::ConversionError::ConversionError(msg) => 
                AuthError::AuthError(format!("Authentication error: {}", msg), StatusCode::UNAUTHORIZED)
        }
    }
}

impl IntoResponse for AuthError{
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
