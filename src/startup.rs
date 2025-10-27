use crate::routes::download::download;
use crate::routes::health_check;
use crate::routes::login::login;
use crate::routes::signup::signup;
use crate::routes::upload::upload_file;
use crate::security::{authenticate, rate_limit};
use axum::extract::ConnectInfo;
use axum::http::HeaderMap;
use axum::routing::{get, post};
use axum::serve;
use axum::{Router, middleware};
use redis::io::tcp::socket2::SockAddr;
use sqlx::PgPool;
use std::net::{IpAddr, SocketAddr};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

pub async fn startup(listener: TcpListener, pg_pool: PgPool) -> Result<(), std::io::Error> {
    // Create app with database connection pool as state
    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/login", post(login))
        .route("/api/signup", post(signup))
        .route(
            "/api/upload",
            post(upload_file).layer(middleware::from_fn_with_state(
                pg_pool.clone(),
                authenticate,
            )),
        )
        .route("/api/download/{*file_link}", get(download))
        .layer(middleware::from_fn(rate_limit))
        .nest_service("/files", ServeDir::new("content"))
        .with_state(pg_pool);

    tracing::info!("Server running on http://0.0.0.0:3000");
    let server = serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    );

    server.await
}
