use axum::http::status;
use axum::Json;
use axum::response::IntoResponse;

#[tracing::instrument]
pub async fn health_check() -> impl IntoResponse {
    tracing::debug!("Health check called");
    Json("Healthy")
}
