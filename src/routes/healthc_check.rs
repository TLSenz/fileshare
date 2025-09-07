use axum::http::status;
use axum::response::IntoResponse;

pub async fn health_check() -> impl IntoResponse {
    status::StatusCode::OK
}