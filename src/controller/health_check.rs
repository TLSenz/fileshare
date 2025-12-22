use axum::http::status;
use axum::response::IntoResponse;

#[tracing::instrument]
pub async fn health_check() -> impl IntoResponse {
    tracing::debug!("Health check called");
    status::StatusCode::OK
}
