use std::fmt::Error;
use std::fs::File;
use std::io::Write;
use bytes::Bytes;
use aws_sdk_s3::primitives::ByteStream;
use axum::extract::Multipart;
use bcrypt::hash;
use sqlx::PgPool;
use crate::model::filemodel::GetFileResponse;
use crate::model::usermodel::{ConversionError, FileToInsert};
use crate::model::usermodel::ConversionError::*;
use crate::repository::filerepository::{check_if_file_name_exists, get_file_name_from_db, write_name_to_db};




