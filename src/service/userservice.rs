use sqlx::PgPool;
use crate::model::usermodel::{ConversionError, SignupRequest, LoginRequest};
use crate::repository::userrepository::{check_if_user_exist_login, create_user as repo_create_user};

pub async fn create_user(pool: PgPool, user: SignupRequest) -> bool {
    let created_user = repo_create_user(pool, user).await;
    created_user.is_ok()
}

pub async fn check_user_login(pool: PgPool, user: LoginRequest) -> Result<bool, ConversionError> {
    if check_if_user_exist_login(pool, user).await? {
        Ok(true)
    } else { 
        Ok(false)
    }
}