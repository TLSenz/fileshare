use crate::configuration::AppState;
use crate::model::FileToInsert;
use aws_config::BehaviorVersion;
use aws_sdk_s3::primitives::ByteStream;
use bytes::Bytes;
use crate::repository::write_name_to_db;

pub async fn upload_aws(app_state: &AppState, file_meta_data: &FileToInsert, data: &Bytes) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    tracing::info!("Writing File Information to Database");
    write_name_to_db(&app_state.pg_pool, *file_meta_data).await?;
    tracing::info!("Uploading File to S3");
    upload_to_s3(data,file_meta_data, app_state.settings.application.aws_settings.bucket_name)?;
    tracing::info!("File Uploaded to S3 Sucessfully");
    Ok(())

}

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
    bucket_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
    let client = aws_sdk_s3::Client::new(&config);

    match client.head_bucket().bucket(&bucket_name.to_string()).send().await {
        Ok(_) => {
          tracing::info!("S3 Bucket already Existed");
          return Ok(())
        } ,
        Err(e) => {
            let service_error = e.into_service_error();
            if service_error.is_not_found() {
                tracing::info!("Creating S3 Bucket because didnot exist");
                client.create_bucket().bucket(&bucket_name.to_string()).send().await?;
                Ok(())
            } else {
                println!("Error: {}", service_error);
                tracing::error!("Error connecting to AWS");
                Err(Box::new(service_error))
            }
        }
    }
}
//pub  async fn get_form_s3() -> Result<(),()>{}
