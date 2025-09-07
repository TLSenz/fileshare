use std::error::Error;

use sqlx::PgPool;
use crate::model::{ConversionError, EncodeJWT, LoginRequest, SignupRequest, User};

pub async fn create_user(pool: PgPool, new_user: SignupRequest) -> Result<(), Box<dyn Error>> {
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
            Ok(())
        }
        Err(err) => {
            println!("Error creating user: {:?}", err);
            Err(Box::new(err))
        }
    }
}

pub async fn check_if_user_exist(pool: PgPool, user: EncodeJWT) -> Result<bool, ConversionError> {
    let result = sqlx::query!(
        r#"
        SELECT COUNT(*) as count
        FROM users
        WHERE email = $1 AND name = $2
        LIMIT 1
        "#,
        user.email,
        user.username
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
        WHERE name = $1 AND password = $2 AND email = $3
        LIMIT 1
        "#,
        user.name,
        user.password,
        user.email
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
