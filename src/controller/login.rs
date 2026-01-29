use crate::configuration::AppState;
use crate::model::{LoginRequest, LoginResponse};
use crate::security::encode_jwt;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use uuid::Uuid;

pub async fn login(
    State(appState): State<AppState>,
    Json(user): Json<LoginRequest>,
) -> impl IntoResponse {
    let request_id = Uuid::new_v4();
    tracing::info!(
        %request_id,
        user_email = %user.email,
        user_name = %user.name,
        "Login request received"
    );

    // Fetch stored password hash for the given user and email
    let result = sqlx::query!(
        r#"
        SELECT password as "password!" 
        FROM users
        WHERE name = $1 AND email = $2
        LIMIT 1
        "#,
        user.name,
        user.email
    )
    .fetch_optional(&appState.pg_pool)
    .await;

    match result {
        Ok(Some(record)) => {
            // Verify plaintext password against stored bcrypt hash
            match bcrypt::verify(&user.password, &record.password) {
                Ok(true) => match encode_jwt(&user.name, &user.email) {
                    Ok(token) => {
                        tracing::info!(
                            %request_id,
                            "Successfully logged in user"
                        );
                        LoginResponse { token }.into_response()
                    }
                    Err(e) => {
                        tracing::error!(
                            %request_id,
                            %e,
                            "Error creating JWT token"
                        );
                        StatusCode::INTERNAL_SERVER_ERROR.into_response()
                    }
                },
                Ok(false) => {
                    tracing::info!(
                        %request_id,
                        "Unauthorized user: invalid credentials"
                    );
                    StatusCode::UNAUTHORIZED.into_response()
                }
                Err(e) => {
                    tracing::error!(
                        %request_id,
                        error = %e,
                        "Error verifying password hash"
                    );
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }
        Ok(None) => {
            tracing::info!(
                %request_id,
                "Unauthorized user: user not found"
            );
            StatusCode::UNAUTHORIZED.into_response()
        }
        Err(e) => {
            tracing::error!(
                %request_id,
                error = %e,
                "Database error, failed to log in user",
            );
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
