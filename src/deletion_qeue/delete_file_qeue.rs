use sqlx::PgPool;
use tokio::sync::mpsc;
use crate::model::DeleteWorkerRequest;

pub fn init_deletion_qeu(pool: &PgPool) -> mpsc::Sender<DeleteWorkerRequest>{

    let (tx, mut rx) = mpsc::channel::<DeleteWorkerRequest>(1000);

    tokio::spawn(async move {
        tracing::info!("Starting Deletion Worker for Request", %request.uuid);
        while let Some(request) = rx.recv().await {
            let uuid = request.uuid;
            tracing::info!("Request received for uuid: {}", uuid);

        }
    });



    todo!()
}