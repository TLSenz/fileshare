use axum::http::StatusCode;
use sqlx::PgPool;
use tokio::sync::mpsc;
use uuid::Uuid;
use crate::model::{DeleteError, DeleteWorkerRequest};
use crate::repository::{delete_file_from_db,  validate_delete_token};
use crate::service::delete_from_s3;

pub fn init_deletion_qeu(pool: &PgPool) -> mpsc::Sender<DeleteWorkerRequest>{

    let (tx, mut rx) = mpsc::channel::<DeleteWorkerRequest>(1000);
    let pool = pool.clone();

    tokio::spawn(async move {
        tracing::info!("Starting Deletion Worker for Request");
        while let Some(request) = rx.recv().await {
            let uuid = request.uuid;
            tracing::info!("Request received for uuid: {}", uuid);
            match delete_file(&pool, request.delete_token.as_str(), &request.bucket_name.as_str(), &request.file_key.as_str(), uuid).await {
                Ok(()) => tracing::info!("File deleted successfully with uuid {}", uuid ),
                Err(DeleteError::AwsError(StatusCode::BAD_GATEWAY)) => {
                    tracing::error!("Error deleting file with uuid {}. S3 Error", uuid);
                },
                Err(DeleteError::DeletionFailed(StatusCode::BAD_GATEWAY)) => {
                    tracing::error!("Error deleting file with uuid {}. Databes Error", uuid);
                },
                _ => tracing::info!("Something Impossible happened")
            }

        }
    });

    tx
}

pub async fn delete_file(pool: &PgPool, delete_token: &str, bucket_name: &str, file_key: &str, request_id: Uuid) -> Result<(), DeleteError> {
    delete_from_s3(file_key.to_string(), bucket_name).await.map_err(|e|{
        tracing::error!("Request Uuid: {} AWS Error: {} ", request_id , e);
        DeleteError::AwsError(StatusCode::BAD_GATEWAY)
    } )?;
    delete_file_from_db(pool, delete_token).await.map_err(|_| DeleteError::DeletionFailed(StatusCode::BAD_GATEWAY))?;
    Ok(())
}