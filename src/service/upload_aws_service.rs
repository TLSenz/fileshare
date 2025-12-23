use crate::configuration::AppState;
use crate::model::FileToInsert;
use aws_config::BehaviorVersion;
use aws_sdk_s3::primitives::ByteStream;
use bytes::Bytes;

pub async fn upload_aws(app_state: AppState, file_meta_data: &FileToInsert, data: &Bytes) {}

#[tracing::instrument(skip(data))]
pub async fn upload_to_s3(
    data: &Bytes,
    data_info: &FileToInsert,
    bucket_name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let client = aws_sdk_s3::Client::new(&config);

    client
        .put_object()
        .bucket(bucket_name)
        .key(&data_info.file_name)
        .body(ByteStream::from(data.to_vec()))
        .send()
        .await?;

    Ok(())
}

pub async fn aws_setup(
    bucket_name: String,
    bucket_region: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
    let client = aws_sdk_s3::Client::new(&config);

    match client.head_bucket().bucket(&bucket_name).send().await {
        Ok(_) => return Ok(()),
        Err(e) => {
            let service_error = e.into_service_error();
            if service_error.is_not_found() {
                client.create_bucket().bucket(&bucket_name).send().await?;
                Ok(())
            } else {
                Err(Box::new(service_error))
            }
        }
    }
}
//pub  async fn get_form_s3() -> Result<(),()>{}
