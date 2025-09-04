use std::env;
use dotenv::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    Ok(pool)
}