use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::__private228::ser::serialize_tagged_newtype;
use sqlx::PgPool;
use uuid::Uuid;
use crate::configuration::build_subscriber;
use crate::model::{LoginRequest, LoginResponse};
use crate::security::encode_jwt;

pub async fn login(
    State(pool): State<PgPool>,
    Json(user): Json<LoginRequest>
) -> impl IntoResponse {
    
    let request_id = Uuid::new_v4();
    tracing::info_span!(
        "User is Logging in.",
        %request_id,
        user_email = %user.email,
        user_name = %user.name
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
                        tracing::info_span!(
                        "Successfully logt in User",
                         %request_id           );
                        LoginResponse { token }.into_response()
                    },
                    Err(_) => {
                        tracing::error_span!(
                            "Error Creating JWT Token",
                            %request_id
                        );
                        StatusCode::INTERNAL_SERVER_ERROR.into_response()
                    } 
                }
            } else {
                tracing::info_span!(
                    "Unauthorized User",
                    %request_id
                );
                StatusCode::UNAUTHORIZED.into_response()
            }
        }
        Err(_) => {
            tracing::error_span!(
                "Error Database, failed to log in in user",
                %request_id,
            );
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        },
    }
}