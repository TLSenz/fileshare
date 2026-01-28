use crate::configuration::AppState;
use crate::model::ConversionError;
use crate::repository::filerepository::get_file_name_from_db;
use crate::service::aws_service::get_from_s3;
use bytes::Bytes;

pub async fn get_file_data(
    app_state: &AppState,
    file_link: &str,
) -> Result<(Bytes, String), ConversionError> {
    let file = get_file_name_from_db(&app_state.pg_pool, file_link)
        .await
        .map_err(|_| ConversionError::ConversionError("File not found in database".to_string()))?;

    if file.on_aws {
        let bucket_name = app_state.settings.application.aws_settings.bucket_name.clone();
        let file_key = file.file_name.clone();
        let stream = get_from_s3(bucket_name, file_key)
            .await
            .map_err(|e| ConversionError::ConversionError(format!("AWS error: {}", e)))?;
        
        let data = stream
            .collect()
            .await
            .map_err(|e| ConversionError::ConversionError(format!("Error collecting stream: {}", e)))?
            .into_bytes();
        
        Ok((data, file.content_type))
    } else {
        let data = tokio::fs::read(&file.storage_path)
            .await
            .map_err(|e| ConversionError::ConversionError(format!("Error reading local file: {}", e)))?;
        
        Ok((Bytes::from(data), file.content_type))
    }
}
