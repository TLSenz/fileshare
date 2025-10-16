use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use sqlx::PgPool;
use uuid::Uuid;
use crate::model::{LoginRequest, LoginResponse};
use crate::security::encode_jwt;

pub async fn login(
    State(pool): State<PgPool>,
    Json(user): Json<LoginRequest>
) -> impl IntoResponse {
    
    let request_id = Uuid::new_v4();
    tracing::info!(
        %request_id,
        user_email = %user.email,
        user_name = %user.name,
        "Login request received"
    );

    let result = sqlx::query!(
        r#"
        SELECT COUNT(*) as count
        FROM users
        WHERE name = $1 AND password = $2 AND email = $3
        LIMIT 1
        "#,
        user.name,
        user.password,
        user.email
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(record) => {
            let count = record.count.unwrap_or(0);
            if count > 0 {
                match encode_jwt(&user.name, &user.email) {
                    Ok(token) => {
                        tracing::info!(
                            %request_id,
                            "Successfully logged in user"
                        );
                        LoginResponse { token }.into_response()
                    },
                    Err(_) => {
                        tracing::error!(
                            %request_id,
                            "Error creating JWT token"
                        );
                        StatusCode::INTERNAL_SERVER_ERROR.into_response()
                    } 
                }
            } else {
                tracing::info!(
                    %request_id,
                    "Unauthorized user"
                );
                StatusCode::UNAUTHORIZED.into_response()
            }
        }
        Err(_) => {
            tracing::error!(
                %request_id,
                "Database error, failed to log in user",
            );
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        },
    }
}