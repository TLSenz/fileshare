use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::PgPool;
use crate::model::{AuthError, LoginRequest, LoginResponse};
use crate::repository::check_if_user_exist_login;
use crate::security::encode_jwt;

pub async fn login(
    State(pool): State<PgPool>,
    Json(user): Json<LoginRequest>
) -> Result<LoginResponse, AuthError> {
    if check_if_user_exist_login(pool, user.clone()).await? {
        let token = encode_jwt(&user.name, user.email.as_str())?;

        let response = LoginResponse {
            token: token
        };

        Ok(response)
    } else {
        Err(AuthError::AuthError("auth failed".to_string(), StatusCode::FORBIDDEN))
    }
}