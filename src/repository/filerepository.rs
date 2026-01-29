use crate::model::{ConversionError, File, FileToInsert};
use sqlx::PgPool;
use std::fmt::Error;

pub async fn write_file_info_to_db(
    pool: &PgPool,
    storing_file: &FileToInsert,
) -> Result<File, Error> {
    let result = sqlx::query_as!(
        File,
        r#"
        INSERT INTO file (
            file_name, hashed_file_name, content_hash, content_type,delete_token,
            size, storage_path, owner_id, is_public, is_deleted, on_aws
        ) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING id, file_name, hashed_file_name, content_hash, content_type,delete_token, size, storage_path, owner_id, is_public as "is_public!", is_deleted as "is_deleted!", on_aws as "on_aws!", created_at, updated_at, deleted_at
        "#,
        &storing_file.file_name,
        &storing_file.hashed_file_name,
        &storing_file.content_hash,
        &storing_file.content_type,
        &storing_file.delete_token,
        &storing_file.size,
        &storing_file.storage_path,
        &storing_file.owner_id as &Option<i32>,
        storing_file.is_public,
        storing_file.is_deleted,
        storing_file.on_aws
    )
        .fetch_one(pool)
        .await;

    match result {
        Ok(file) => {
            println!("File inserted: {:?}", file);
            Ok(file)
        }
        Err(err) => {
            println!("Database Error: {}", err);
            Err(Error)
        }
    }
}

pub async fn get_file_name_from_db(pool: &PgPool, file_name: &str) -> Result<File, Error> {
    println!("{}", file_name);
    let result = sqlx::query_as!(
        File,
        r#"
        SELECT id, file_name, hashed_file_name, content_hash, content_type,delete_token, size, storage_path, owner_id, is_public as "is_public!", is_deleted as "is_deleted!", on_aws as "on_aws!", created_at, updated_at, deleted_at FROM file
        WHERE hashed_file_name = $1 
        LIMIT 1
        "#,
        file_name
    )
        .fetch_one(pool)
        .await;

    match result {
        Ok(file) => Ok(file),
        Err(err) => {
            println!("Database Error: {}", err);
            Err(Error)
        }
    }
}

pub async fn check_if_file_name_available(
    pool: &PgPool,
    name: String,
) -> Result<bool, ConversionError> {
    let result = sqlx::query!(
        r#"
        SELECT COUNT(*) as count FROM file 
        WHERE file_name = $1
        "#,
        name
    )
    .fetch_one(pool)
    .await?;

    // If count is 0, the file name doesn't exist
    Ok(result.count.unwrap_or(0) == 0)
}

pub async fn delete_file_from_db(pool: &PgPool, delete_token: &str) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query!("DELETE FROM file WHERE delete_token = $1", delete_token)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
}

pub async fn set_file_deleted(pool: &PgPool, delete_token: &str) -> Result<(), sqlx::Error> {
    sqlx::query!("UPDATE File set is_deleted = true where delete_token = $1", delete_token)
        .execute(pool)
        .await?;
}

pub async fn validate_delete_token(
    pool: &PgPool,
    delete_token: &str,
) -> Result<bool, sqlx::Error> {

    let record = sqlx::query!(
        "SELECT count(*) FROM file WHERE delete_token = $1",
        delete_token
    )
        .fetch_one(pool)
        .await?;

    Ok(record.count == Some(1))
}
