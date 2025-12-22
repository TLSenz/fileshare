use aws_sdk_s3::primitives::ByteStream;
use bytes::Bytes;
use crate::model::FileToInsert;

#[tracing::instrument(skip(data))]
pub async fn aws(data: &Bytes, data_info: &FileToInsert) -> Result<(), Box<dyn std::error::Error>> {
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