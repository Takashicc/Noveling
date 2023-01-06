use config::ConfigError;
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub mongo_uri: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let cfg = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        cfg.try_deserialize()
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenv().expect("Cannot found .env file");
    Config::from_env().expect("Failed to load config")
});

pub const MESSAGE_EMAIL_PASSWORD_INCORRECT: &str = "Email not found or Password incorrect";
pub const MESSAGE_USER_ALREADY_EXISTS: &str = "User already exists";
pub const MESSAGE_UNEXPECTED_ERROR: &str = "An unexpected error has occurred";
pub const MESSAGE_AUTHORIZATION_HEADER_MISSING: &str = "Authorization header missing";
pub const MESSAGE_INVALID_TOKEN_TYPE: &str = "Invalid token type";
pub const MESSAGE_TOKEN_TYPE_MISSING: &str = "Token type missing";
pub const MESSAGE_TOKEN_MISSING: &str = "Token missing";
