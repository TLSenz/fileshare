use std::error::Error;
use std::net::Shutdown::Read;
use sqlx::PgPool;
use crate::model::{File, FileDTO, GetFileResponse, User};
use crate::repository::get_files;

pub async fn read_files(user_id: i32, pool: PgPool) -> Result<Vec<FileDTO>, Box<dyn  Error>> {

    let list_of_files = get_files(pool, user_id).await?;
    let list_of_files: Vec<FileDTO> = list_of_files
        .into_iter()
        .map(FileDTO::from)
        .collect();
    Ok(list_of_files)
    

}
