use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    println!("Connecting to database: {}", database_url);
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    println!("Successfully connected to the database!");
    
    // Test a simple query
    let row: (i64,) = sqlx::query_as("SELECT $1::BIGINT", )
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;
    
    println!("Query result: {}", row.0);
    
    Ok(())
}