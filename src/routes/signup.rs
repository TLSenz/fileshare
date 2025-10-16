use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sqlx::PgPool;
use uuid::Uuid;
use crate::model::usermodel::SignupRequest;

pub async fn signup(
    State(pool): State<PgPool>,
    Json(user): Json<SignupRequest>
) -> Result<impl IntoResponse, StatusCode> {

    let request_id = Uuid::new_v4();
    tracing::info_span!(
        "User is Logging in.",
        %request_id,
        user_email = %user.email,
        user_name = %user.name,
        "Signup request received"
    );

    sqlx::query!(
        r#"
        INSERT INTO users (name, email, password)
        VALUES ($1, $2, $3)
        "#,
        user.name,
        user.email,
        user.password
    )
    .execute(&pool)
    .await
    .map_err(|_| {
        tracing::error!(
            %request_id,
            "Error signing up user"
        );
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tracing::info!(
        %request_id,
        "Successfully signed up user"
    );
    Ok(StatusCode::OK)
}