use crate::model::FileToInsert;
use aws_sdk_s3::primitives::ByteStream;
use bytes::Bytes;
use sqlx::PgPool;

pub async fn upload_aws(pg_pool: &PgPool, mut file_meta_data: FileToInsert, data: &Bytes) {}

#[tracing::instrument(skip(data))]
pub async fn upload_to_s3(
    data: &Bytes,
    data_info: &FileToInsert,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let client = aws_sdk_s3::Client::new(&config);

    client
        .put_object()
        .bucket("fileshareapistorage")
        .key(&data_info.file_name)
        .body(ByteStream::from(data.to_vec()))
        .send()
        .await?;

    Ok(())
}
