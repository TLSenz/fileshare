use axum::http::StatusCode;
use sqlx::PgPool;
use tokio::sync::mpsc;
use crate::model::{DeleteError, DeleteWorkerRequest};
use crate::repository::{set_file_deleted, validate_delete_token};
use crate::service::delete_from_s3;

pub fn init_deletion_qeu(pool: &PgPool) -> mpsc::Sender<DeleteWorkerRequest>{

    let (tx, mut rx) = mpsc::channel::<DeleteWorkerRequest>(1000);

    tokio::spawn(async move {
        tracing::info!("Starting Deletion Worker for Request", %request.uuid);
        while let Some(request) = rx.recv().await {
            let uuid = request.uuid;
            tracing::info!("Request received for uuid: {}", uuid);
            delete_file

        }
    });

    tx
}




pub async fn delete_file(pool: &PgPool, delete_token: &str) -> Result<(), sqlx::Error> {
    let does_file_exist = validate_delete_token(pool, delete_token).await;
    match  {
        Ok(true) => {
            set_file_deleted(pool, delete_token).await.map_err(|_| DeleteError::DeletionFailed(StatusCode::NOT_FOUND))?;
            match f {  }delete_from_s3(file_key.to_string(), bucket_name).await.map_err(|_| DeleteError::DeletionFailed(StatusCode::BAD_GATEWAY))?;
        },
        Ok(false) => {
            return Err(DeleteError::NotFound(StatusCode::NOT_FOUND));
        },
        Err(_) => {
            return Err(DeleteError::DeletionFailed(StatusCode::INTERNAL_SERVER_ERROR));
        }
    }.await.expect("TODO: panic message");
}