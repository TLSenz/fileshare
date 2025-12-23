use crate::configuration::AppState;
use crate::model::usermodel::SignupRequest;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use regex::Regex;
use sqlx::PgPool;
use std::env;
use uuid::Uuid;

#[tracing::instrument(skip(pool, user), fields(request_id = %Uuid::new_v4(), user_email = %user.email, user_name = %user.name))]
pub async fn signup(
    State(appState): State<AppState>,
    Json(user): Json<SignupRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let request_id = Uuid::new_v4();
    tracing::info!("Signup request received");

    let regex_email_match =
        Regex::new(r"^([a-zA-Z0-9_\-\.]+)@([a-zA-Z0-9_\-\.]+)\.([a-zA-Z]{2,6})$").unwrap();

    if user.name.is_empty() || user.password.is_empty() || !regex_email_match.is_match(&user.email)
    {
        tracing::error!({
            %request_id,
            "User Credentials not in right Format. Username and Password cannot be empty and Email must be valid"
        });
        return Err(StatusCode::BAD_REQUEST);
    }
    let hashed_password =
        bcrypt::hash(user.password, 12).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    tracing::debug!(%request_id, "Password hashed for new user");

    sqlx::query!(
        r#"
        INSERT INTO users (name, email, password)
        VALUES ($1, $2, $3)
        "#,
        user.name,
        user.email,
        hashed_password
    )
    .execute(&appState.pg_pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "Error signing up user");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tracing::info!("Successfully signed up user");
    Ok(StatusCode::OK)
}
