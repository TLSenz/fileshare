use crate::security::authenticate;
use axum::{middleware, Router};
use axum::routing::{get, post};
use axum::serve;
use axum::serve::Serve;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use crate::db::create_pool;
use crate::routes::download::download;
use crate::routes::login::login;
use crate::routes::signup::signup;
use crate::routes::upload::upload_file;

pub async fn startup() -> Result<Serve<TcpListener, Router,Router>, std::io::Error> {

    let pool = create_pool().await.expect("Failed to create database pool");

    // Create app with database connection pool as state
    let app = Router::new()
        .route("/api/login", post(login))
        .route("/api/signup", post(signup))
        .route("/api/upload", post(upload_file).layer(middleware::from_fn_with_state(pool.clone(), authenticate)))
        .route("/api/download/{file_link}", get(download))
        .nest_service("/files", ServeDir::new("content"))
        .with_state(pool);

    println!("Server running on http://0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let server = serve(listener, app);

    Ok(server)
}