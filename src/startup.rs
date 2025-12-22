use std::net::{IpAddr, SocketAddr};
use crate::security::{authenticate, rate_limit};
use axum::{middleware, Router};
use axum::extract::ConnectInfo;
use axum::http::HeaderMap;
use axum::routing::{get, post};
use axum::serve;
use redis::io::tcp::socket2::SockAddr;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use crate::controller::download;
use crate::controller::health_check;
use crate::controller::login;
use crate::controller::signup;
use crate::controller::upload_file;

pub async fn startup(listener: TcpListener, pg_pool: PgPool) -> Result<(), std::io::Error> { 
    // Create app with database connection pool as state
    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/login", post(login))
        .route("/api/signup", post(signup))
        .route("/api/upload", post(upload_file).layer(middleware::from_fn_with_state(pg_pool.clone(), authenticate)))
        .route("/api/download/{*file_link}", get(download))
        .layer(middleware::from_fn(rate_limit))
        .nest_service("/files", ServeDir::new("content"))
        .with_state(pg_pool);

    tracing::info!("Server running on http://0.0.0.0:3000");
    let server = serve(listener, app.into_make_service_with_connect_info::<SocketAddr>());

    server.await
}