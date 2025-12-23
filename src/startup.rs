use crate::configuration::{AppState, get_config};
use crate::controller::download;
use crate::controller::health_check;
use crate::controller::login;
use crate::controller::signup;
use crate::controller::upload_file;
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
    let configuration = get_config().expect("Failde to start. Could not Read Config");
    let state = AppState::new(pg_pool, configuration);
    // If Application gets to this Point, the File Has already been Read one Time
    // Create app with database connection pool as state
    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/login", post(login))
        .route("/api/signup", post(signup))
        .route(
            "/api/upload",
            post(upload_file).layer(middleware::from_fn_with_state(state.clone(), authenticate)),
        )
        .route("/api/download/{*file_link}", get(download))
        .layer(middleware::from_fn(rate_limit))
        .with_state(state.clone());

    tracing::info!("Server running on http://0.0.0.0:3000");
    let server = serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    );

    server.await
}
