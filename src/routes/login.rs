use std::error::Error;
use axum::extract::State;
use axum::http::{Response, StatusCode};
use axum::Json;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;
use crate::configuration::build_subscriber;
use crate::model::{AuthError, LoginRequest, LoginResponse, SignupRequest};
use crate::repository::check_if_user_exist_login;
use crate::security::encode_jwt;

pub async fn login(
    State(pool): State<PgPool>,
    Json(user): Json<LoginRequest>
) -> HttpResponse {

        let request_id = Uuid::new_v4();

        let request_span = tracing::info_span!(
            "Login Request",
            %request_id,
            user_email = %user.email,
            user_name = %user.name);
        let _request_span_guard = request_span.enter();
        
      

    

        let response = LoginResponse {
            token: token
        };

    
        match  sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (name, email, password)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
        new_user.name,
        new_user.email,
        new_user.password
    )
            .fetch_one(&pool)
            .instrument(request_span)
            .await
        {
            Ok(_) => {
                encode_jwt(&user.name, &user.email).expect("Could Not Create JWT Token")
                Response::builder().status(200).body(response)
            }
        }

        
    
}