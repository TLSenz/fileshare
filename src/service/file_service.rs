use std::error::Error;
use sqlx::PgPool;
use crate::model::{File, User};
use crate::repository::get_files;

pub async fn read_files(userId: i32, pool: PgPool) -> Result<Vec<File>, dyn  Error> {

    let list_of_files = get_files(pool, userId).await?;
    let list_of_files = list_of_files.try_into();

}
