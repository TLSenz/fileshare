use crate::security::authenticate;
use axum::{middleware, Router};
use axum::routing::{get, post};
use axum::serve;
use axum::serve::Serve;
use serde::__private::de::TagContentOtherField;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use crate::db::create_pool;
use crate::routes::download::download;
use crate::routes::health_check;
use crate::routes::login::login;
use crate::routes::signup::signup;
use crate::routes::upload::upload_file;

pub async fn startup(listener: TcpListener, pg_pool: PgPool) -> Result<Serve<TcpListener, Router,Router>, std::io::Error> { 
    // Create app with database connection pool as state
    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/login", post(login))
        .route("/api/signup", post(signup))
        .route("/api/upload", post(upload_file).layer(middleware::from_fn_with_state(pg_pool.clone(), authenticate)))
        .route("/api/download/{file_link}", get(download))
        .nest_service("/files", ServeDir::new("content"))
        .with_state(pg_pool);

    println!("Server running on http://0.0.0.0:3000");
    let server = serve(listener, app);

    Ok(server)
}