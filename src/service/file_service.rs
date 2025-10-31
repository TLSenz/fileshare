use crate::model::FileDTO;
use crate::repository::get_files;
use sqlx::PgPool;
use std::error::Error;

pub async fn read_files(user_id: i32, pool: PgPool) -> Result<Vec<FileDTO>, Box<dyn Error>> {
    let list_of_files = get_files(pool, user_id).await?;
    let list_of_files: Vec<FileDTO> = list_of_files.into_iter().map(FileDTO::from).collect();
    Ok(list_of_files)
}