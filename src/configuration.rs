use config::ConfigError;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing_subscriber::EnvFilter;

#[derive(Deserialize, Serialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct DatabaseSettings {
    pub host: String,
    pub username: String,
    pub password: String,
    pub name: String,
    pub port: Option<u16>,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
    pub log_level: LogLevel,
    pub log_format: LogFormat,
    pub ttl: i32,
    pub rate_limit: i32,
    pub aws_settings: AWSConfiguration,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AWSConfiguration {
    pub s3_enabled: bool,
    pub bucket_name: String,
    pub region: String,
}

#[derive(Clone)]
pub struct AppState {
    pub pg_pool: PgPool,
    pub settings: Settings,
}

pub fn get_config() -> Result<Settings, ConfigError> {
    let config_file_path = std::env::current_dir()
        .expect("Error getting current die to find configuration.yaml")
        .join("configuration.yaml");
    let settings = config::Config::builder()
        .add_source(config::File::from(config_file_path))
        .build()?;
    settings.try_deserialize::<Settings>()
}

pub fn build_subscriber(log_format: LogFormat, filter: EnvFilter) {
    match log_format {
        LogFormat::Compact => tracing_subscriber::fmt()
            .with_env_filter(filter)
            .compact()
            .init(),
        LogFormat::Full => tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_level(true)
            .with_target(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .compact()
            .init(),
        LogFormat::Pretty => tracing_subscriber::fmt()
            .with_env_filter(filter)
            .pretty()
            .init(),
        LogFormat::Json => tracing_subscriber::fmt()
            .with_env_filter(filter)
            .json()
            .init(),
    }
}

impl Settings {
    pub fn connection_string_database(&self) -> String {
        if self.database.port.is_none() {
            format!(
                "postgres://{}:{}@{}",
                self.database.username, self.database.password, self.database.host
            )
        } else {
            format!(
                "postgres://{}:{}@{}:{}/{}",
                self.database.username,
                self.database.password,
                self.database.host,
                self.database.port.unwrap(),
                self.database.name
            )
        }
    }

    pub fn connection_string_application(&self) -> String {
        format!("{}:{}", self.application.host, self.application.port)
    }
}
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum LogLevel {
    Trace,
    Info,
    Debug,
    Warn,
    Error,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum LogFormat {
    Compact,
    Full,
    Pretty,
    Json,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Info => "info",
            LogLevel::Debug => "debug",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
            LogLevel::Trace => "trace",
        }
    }
}

impl AppState {
    pub fn new(pg_pool: sqlx::PgPool, settings: Settings) -> Self {
        Self { pg_pool, settings }
    }
}
