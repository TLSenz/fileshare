use axum::{middleware, routing::{get, }, Router};
use axum::routing::post;
use axum::extract::State;
use tower_http::services::ServeDir;
use crate::routes::download::download;
use crate::routes::upload::upload_file;
use crate::routes::login::login;
use crate::routes::signup::signup;
use crate::Security::jwt::authenticate;
use crate::db::create_pool;
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    // Create database connection pool
    let pool = create_pool().await.expect("Failed to create database pool");

    // Create app with database connection pool as state
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/api/login", post(login))
        .route("/api/signup", post(signup))
        .route("/api/upload", post(upload_file).layer(middleware::from_fn_with_state(pool.clone(), authenticate)))
        .route("/api/download/{file_link}", get(download))
        .nest_service("/files", ServeDir::new("content"))
        .with_state(pool);

    println!("Server running on http://0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> &'static str{
    "Hello World".as_ref()
}


pub mod routes{
    pub mod download;
    pub mod upload;
    pub mod login;
    pub mod signup;
}
pub mod model{
    pub mod usermodel;
    pub mod filemodel;
    pub mod securitymodel;
}
pub mod repository{
    pub mod userrepository;
    pub mod filerepository;
}
pub mod service{
    pub mod userservice;
    pub mod fileservice;
}
pub mod Security{
    pub mod jwt;
}
pub mod db;
