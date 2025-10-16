
use sqlx::PgPool;
use crate::model::{ConversionError, EncodeJWT, LoginRequest};



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
