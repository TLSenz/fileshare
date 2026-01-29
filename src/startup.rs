use crate::configuration::{AppState, get_config};
use crate::controller::download;
use crate::controller::health_check;
use crate::controller::login;
use crate::controller::signup;
use crate::controller::upload::delete_file;
use crate::controller::upload_file;
use crate::security::{authenticate, rate_limit};
use crate::service::aws_setup;
use axum::routing::{delete, get, post};
use axum::serve;
use axum::{Router, middleware};
use sqlx::PgPool;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub async fn startup(listener: TcpListener, pg_pool: PgPool) -> Result<(), std::io::Error> {
    let configuration = get_config().expect("Failde to start. Could not Read Config");
    let state = AppState::new(pg_pool, configuration.clone());
    if configuration.application.aws_settings.s3_enabled {
        aws_setup(&configuration.application.aws_settings.bucket_name)
            .await
            .map_err(|e| {
                std::io::Error::new(std::io::ErrorKind::Other, format!("AWS Error: {}", e))
            })?;
    }

    // If Application gets to this Point, the File Has already been Read one Time
    // Create app with database connection pool as state
    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/login", post(login))
        .route("/api/signup", post(signup))
        .route(
            "/api/upload",
            post(upload_file).layer(middleware::from_fn_with_state(
                state.pg_pool.clone(),
                authenticate,
            )),
        )
        .route("/api/delete/:id", delete(delete_file))
        .route("/api/download/{*file_link}", get(download))
        .layer(middleware::from_fn(rate_limit))
        .with_state(state.clone());

    tracing::info!(
        "Service running on http://{}:{}",
        state.settings.application.host,
        state.settings.application.port
    );
    let server = serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    );

    server.await
}
