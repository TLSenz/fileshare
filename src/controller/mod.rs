// Controller layer: actual handlers live here now, preserving original logic.

pub mod download;
pub mod login;
pub mod signup;
pub mod upload;
pub mod health_check;

pub use download::download;
pub use login::login;
pub use signup::signup;
pub use upload::{upload_file, create_link};
pub use health_check::health_check;
