use serde::{Deserialize, Serialize};

pub struct GetFileResponse{
    pub(crate) filename: String,
     pub(crate) filepath: String
}

#[derive(Serialize, Deserialize)]
pub struct UploadOptions {
    pub aws_upload: Option<bool>,
    pub local_upload: Option<bool>
}