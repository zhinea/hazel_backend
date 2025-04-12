use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: Option<String>,
    pub dbname: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigApp {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

impl ConfigApp {
    pub fn new() -> Result(Self, ConfigError){
        let s = Config::builder()
            .add_source(File::with_name("config/default").required(false))
            .add_source(config::Environment::with_prefix("APP"))
            .build()?
            .try_deserialize();
    }
}