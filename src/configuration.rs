use aws_config::imds::client::error::FailedToLoadToken;
use config::ConfigError;
use serde::{Deserialize, Serialize};
use tracing_subscriber::EnvFilter;

#[derive(Deserialize,Serialize)]
pub struct Settings{
    pub application: ApplicationSettings,
    pub database: DatabaseSettings
}
#[derive(Deserialize,Serialize)]
pub struct DatabaseSettings{
    pub host: String,
    pub username: String,
    pub password: String,
    pub name: String,
    pub port: Option<u16>
}
#[derive(Deserialize,Serialize)]
pub struct ApplicationSettings{
    pub host: String,
    pub port: u16,
    pub log_level: LogLevel,
    pub log_format: LogFormat,
    pub ttl: i32,
    pub rate_limit: i32
}



pub fn get_config() ->Result<Settings, ConfigError>{

    let config_file_path = std::env::current_dir()
        .expect("Error getting current die to find configuration.yaml")
        .join("configuration.yaml");
    let settings = config::Config::builder().add_source(config::File::from(config_file_path)).build()?;
    settings.try_deserialize::<Settings>()
}


pub fn build_subscriber(log_format: LogFormat, filter: EnvFilter) {
    match log_format {
        LogFormat::Compact => {
            tracing_subscriber::fmt()
                .with_env_filter(filter)
                .compact()
                .init()
        }
        LogFormat::Full => {
            tracing_subscriber::fmt()
                .with_env_filter(filter)
                .with_level(true)
                .with_target(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .compact()
                .init()
        }
        LogFormat::Pretty => {
            tracing_subscriber::fmt()
                .with_env_filter(filter)
                .pretty()
                .init()
        }
        LogFormat::Json => {
            tracing_subscriber::fmt()
                .with_env_filter(filter)
                .json()
                .init()
        }
    }
}



impl Settings {

    /// Build a PostgreSQL connection string from the configured database settings.
    ///
    /// When `database.port` is `None`, the returned URI omits both the port and the database name.
    /// When `database.port` is `Some(p)`, the returned URI includes the port and the database name.
    ///
    /// # Examples
    ///
    /// ```
    /// let db = DatabaseSettings {
    ///     host: "localhost".into(),
    ///     username: "user".into(),
    ///     password: "pass".into(),
    ///     name: "mydb".into(),
    ///     port: Some(5432),
    /// };
    /// let app = ApplicationSettings {
    ///     host: "127.0.0.1".into(),
    ///     port: 8080,
    ///     log_level: LogLevel::Info,
    ///     log_format: LogFormat::Compact,
    ///     ttl: 60,
    ///     rate_limit: 100,
    /// };
    /// let settings = Settings { application: app, database: db };
    /// assert_eq!(settings.connection_string_database(), "postgres://user:pass@localhost:5432/mydb");
    /// ```
    pub fn connection_string_database(&self) -> String {

        if self.database.port.is_none(){
            format!(
                "postgres://{}:{}@{}",
                self.database.username, self.database.password, self.database.host)
        }
        else {
            format!(
                "postgres://{}:{}@{}:{}/{}",
                self.database.username, self.database.password, self.database.host, self.database.port.unwrap(), self.database.name)
        }

    }

    /// Builds the application's listening address as `host:port`.
    ///
    /// The returned string uses the form `host:port`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::{Settings, ApplicationSettings, DatabaseSettings, LogLevel, LogFormat};
    ///
    /// let settings = Settings {
    ///     application: ApplicationSettings {
    ///         host: "127.0.0.1".to_string(),
    ///         port: 8080,
    ///         log_level: LogLevel::Info,
    ///         log_format: LogFormat::Compact,
    ///         ttl: 60,
    ///         rate_limit: 100,
    ///     },
    ///     database: DatabaseSettings {
    ///         host: "db".to_string(),
    ///         username: "user".to_string(),
    ///         password: "pass".to_string(),
    ///         name: "app_db".to_string(),
    ///         port: Some(5432),
    ///     },
    /// };
    ///
    /// assert_eq!(settings.connection_string_application(), "127.0.0.1:8080");
    /// ```
    pub  fn connection_string_application(&self) -> String{
        format!("{}:{}",self.application.host,self.application.port)
    }

}
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum LogLevel {
    Trace,
    Info,
    Debug,
    Warn,
    Error
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum LogFormat{
    Compact,
    Full,
    Pretty,
    Json

}

impl LogLevel {
   pub  fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Info => "info",
            LogLevel::Debug => "debug",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
            LogLevel::Trace => "trace",
        }
    }
}