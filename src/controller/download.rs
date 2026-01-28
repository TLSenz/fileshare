use crate::configuration::AppState;
use crate::service::get_file_data;
use axum::body::*;
use axum::extract::{Path, State};
use axum::http::{Response, StatusCode, header};
use axum::response::IntoResponse;
use uuid::Uuid;

pub async fn download(
    State(app_state): State<AppState>,
    Path(file_link): Path<String>,
) -> impl IntoResponse {
    let request_id = Uuid::new_v4();
    tracing::info!(
        %request_id,
        file_link = %file_link,
        "Downloading file"
    );

    match get_file_data(&app_state, &file_link).await {
        Ok((data, content_type)) => {
            tracing::info!(%request_id, bytes = data.len(), "Sending file");
            let body = Body::from(data);

            Response::builder()
                .header(
                    header::CONTENT_TYPE,
                    if content_type.contains('/') {
                        content_type.to_string()
                    } else {
                        format!("application/{}", content_type)
                    }
                )
                .body(body)
                .unwrap()
        }
        Err(e) => {
            tracing::error!(
                %request_id,
                error = %e,
                "Error getting file data"
            );
            (StatusCode::INTERNAL_SERVER_ERROR, "AWS Stream Error").into_response()
        }
    }
}
