// Controller layer: actual handlers live here now, preserving original logic.

pub mod download;
pub mod health_check;
pub mod login;
pub mod signup;
pub mod upload;

pub use download::download;
pub use health_check::health_check;
pub use login::login;
pub use signup::signup;
pub use upload::{create_link, upload_file};
