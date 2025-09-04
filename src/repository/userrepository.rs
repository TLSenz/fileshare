use std::fmt::Error;
use sqlx::PgPool;
use crate::model::securitymodel::EncodeJWT;
use crate::model::usermodel::{ConversionError, CreateUserRequest, LoginRequest, User};

pub async fn create_user(pool: PgPool, new_user: CreateUserRequest) -> Result<bool, Error> {
    let result = sqlx::query_as!(
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
    .await;
    
    match result {
        Ok(user) => {
            println!("User created: {:?}", user);
            Ok(true)
        }
        Err(err) => {
            println!("Database Error: {}", err);
            Ok(false)
        }
    }
}

pub async fn check_if_user_exist(pool: PgPool, user: EncodeJWT) -> Result<bool, ConversionError> {
    let result = sqlx::query!(
        r#"
        SELECT COUNT(*) as count
        FROM users
        WHERE email = $1
        LIMIT 1
        "#,
        user.email
    )
    .fetch_one(&pool)
    .await?;
    
    let count = result.count.unwrap_or(0);
    if count > 0 {
        Ok(true)
    } else {
        Err(ConversionError::ConversionError("User not found".to_string()))
    }
}

pub async fn check_if_user_exist_login(pool: PgPool, user: LoginRequest) -> Result<bool, ConversionError> {
    let result = sqlx::query!(
        r#"
        SELECT COUNT(*) as count
        FROM users
        WHERE name = $1 AND password = $2
        LIMIT 1
        "#,
        user.name,
        user.password
    )
    .fetch_one(&pool)
    .await?;
    
    let count = result.count.unwrap_or(0);
    if count > 0 {
        Ok(true)
    } else {
        Err(ConversionError::ConversionError("Invalid credentials".to_string()))
    }
}