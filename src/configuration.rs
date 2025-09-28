use std::io::Error;
use config::{Config, ConfigError};
use serde::{Deserialize, Serialize};

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
    pub port: u16
}
#[derive(Deserialize,Serialize)]
pub struct ApplicationSettings{
    pub host: String,
    pub port: u16
}



pub fn get_config() ->Result<Settings, ConfigError>{

    let config_file_path = std::env::current_dir()
        .expect("Error getting current die to find configuration.yaml")
        .join("configuration.yaml");
    let settings = config::Config::builder().add_source(config::File::from(config_file_path)).build()?;
    settings.try_deserialize::<Settings>()
}

impl Settings {

    pub fn connection_string_database(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.database.username, self.database.password, self.database.host, self.database.port, self.database.name
        )
    }

    pub  fn connection_string_application(&self) -> String{
        format!("{}:{}",self.application.host,self.application.port)
    }

}