use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::PgPool;
use crate::model::securitymodel::AuthError;
use crate::model::usermodel::{LoginRequest, LoginResponse};
use crate::repository::userrepository::check_if_user_exist_login;
use crate::Security::jwt::encode_jwt;
use crate::service::userservice::check_user_login;

pub async fn login(
    State(pool): State<PgPool>,
    Json(user): Json<LoginRequest>
) -> Result<LoginResponse, AuthError> {
    if check_if_user_exist_login(pool, user.clone()).await? {
        let token = encode_jwt(&user.name, user.email.as_str())?;

        let response = LoginResponse {
            status_code: StatusCode::OK,
            jwt_token: token
        };

        Ok(response)
    } else {
        Err(AuthError::AuthError("auth failed".to_string(), StatusCode::FORBIDDEN))
    }
}