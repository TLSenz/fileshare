pub struct GetFileResponse{
    pub(crate) filename: String,
     pub(crate) filepath: String
}

pub struct FileUploadOptions {
    pub is_public: Option<bool>,
    pub is_aws: Option<bool>
}