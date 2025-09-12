use std::io::Error;
use config::{Config, ConfigError};
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct Settings{
    application_settings: ApplicationSettings,
    database_settings: DatabaseSettings
}
#[derive(Deserialize,Serialize)]
pub struct DatabaseSettings{
    host: String,
    username: String,
    password: String,
    name: String,
    port: u16
}
#[derive(Deserialize,Serialize)]
pub struct ApplicationSettings{
    host: u16,
    port: u16
}



pub fn get_config() ->Result<Settings, ConfigError>{

    let config_file_path = std::env::current_dir()
        .expect("Error getting current die to find configuration.yaml")
        .join("configuration.yaml");
    let settings = config::Config::builder().add_source(config::File::from(config_file_path)).build()?;
    settings.try_deserialize::<Settings>()
}

impl Settings {

    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.database_settings.name, self.database_settings.password, self.database_settings.host, self.database_settings.port, self.database_settings.name
        )
    }

}