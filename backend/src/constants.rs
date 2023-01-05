use config::ConfigError;
use dotenv::dotenv;
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub mongo_uri: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let cfg = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        Ok(cfg.try_deserialize()?)
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenv().expect("Cannot found .env file");
    Config::from_env().expect("Failed to load config")
});
