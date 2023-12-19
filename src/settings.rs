use std::env;
use std::path::PathBuf;
use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Mongodb {
    pub host: String,
    pub db_name: String,
    pub password: String,
    pub user_name: String,
    pub port: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings{
    pub mongodb: Mongodb,
}

impl Mongodb {
    pub fn connection_string(&self) -> String {
        format!(
            "mongodb://{}:{}@{}:{}",
            self.user_name, self.password, self.host, self.port
        )
    }
}

impl Settings {
    pub fn new(base_path: PathBuf) -> Result<Self, ConfigError> {

        // Start off by merging in the "default" configuration file
        let path = base_path.join("configuration/base");
        println!("{}", path.to_str().unwrap());

        // Detect the running environment
        let environment = env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".into());

        // Add in environment-specific settings (optional)
        let path = base_path.join(&format!("configuration/{}", environment));

        // Build the configuration
        let config = Config::builder()
            .add_source(File::with_name(path.to_str().unwrap()))
            .add_source(File::with_name(path.to_str().unwrap()).required(false))
            .add_source(config::Environment::default().separator("_")).build()?;

        // Deserialize (and thus freeze) the entire configuration
        config.try_deserialize()
    }
}
