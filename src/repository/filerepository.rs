use crate::model::{ConversionError, File, FileToInsert};
use sqlx::PgPool;
use std::fmt::Error;

pub async fn write_name_to_db(pool: &PgPool, storing_file: FileToInsert) -> Result<File, Error> {
    let result = sqlx::query_as!(
        File,
        r#"
        INSERT INTO file (
            file_name, hashed_file_name, content_hash, content_type, 
            size, storage_path, owner_id, is_public, is_deleted
        ) 
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#,
        storing_file.file_name,
        storing_file.hashed_file_name,
        storing_file.content_hash,
        storing_file.content_type,
        storing_file.size,
        storing_file.storage_path,
        storing_file.owner_id,
        storing_file.is_public,
        storing_file.is_deleted
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
        SELECT * FROM file 
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

pub async fn check_if_file_name_exists(
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
