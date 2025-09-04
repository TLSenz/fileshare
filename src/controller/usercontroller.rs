use axum::http::StatusCode;
use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use sqlx::PgPool;
use crate::model::securitymodel::AuthError;
use crate::model::usermodel::{CreateUserRequest, LoginRequest, LoginResponse};
use crate::Security::jwt::encode_jwt;
use crate::service::userservice::{check_user_login, create_user};

pub async fn signup(
    State(pool): State<PgPool>,
    Json(user): Json<CreateUserRequest>
) -> impl IntoResponse {
    let result = create_user(pool, user).await;
    
    if result {
        StatusCode::OK
    } else { 
        StatusCode::CONFLICT
    }
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(user): Json<LoginRequest>
) -> Result<LoginResponse, AuthError> {
    if check_user_login(pool, user.clone()).await? {
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