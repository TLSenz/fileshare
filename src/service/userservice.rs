use sqlx::PgPool;
use crate::model::usermodel::{ConversionError, CreateUserRequest, LoginRequest, User};
use crate::repository::userrepository::{check_if_user_exist_login, create_user as repo_create_user};

pub async fn create_user(pool: PgPool, user: CreateUserRequest) -> bool {
    let created_user = repo_create_user(pool, user).await;
    match created_user {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn check_user_login(pool: PgPool, user: LoginRequest) -> Result<bool, ConversionError> {
    if check_if_user_exist_login(pool, user).await? {
        Ok(true)
    } else { 
        Ok(false)
    }
}