use config::ConfigError;
use dotenv::dotenv;
use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
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

lazy_static! {
    pub static ref CONFIG: Config = {
        dotenv().expect("Cannot found .env file");

        Config::from_env().expect("Failed to load config")
    };
}
