use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sqlx::PgPool;
use crate::model::usermodel::CreateUserRequest;
use crate::repository::userrepository::create_user;

pub async fn signup(
    State(pool): State<PgPool>,
    Json(user): Json<CreateUserRequest>
) -> Result<impl IntoResponse,StatusCode> {
    let result = create_user(pool, user).await.map_err(|e| 
    StatusCode::INTERNAL_SERVER_ERROR
    )?;
    Ok(StatusCode::OK)
    
}