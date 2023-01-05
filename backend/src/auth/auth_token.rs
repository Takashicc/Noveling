use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::{constants, errors::AppError};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    iat: i64,
    exp: i64,
    user_id: String,
}

impl Claims {
    pub fn generate_token(id: Option<ObjectId>) -> Result<String, AppError> {
        let user_id = match id {
            Some(v) => v.to_string(),
            None => return Err(AppError::server_error(constants::MESSAGE_UNEXPECTED_ERROR)),
        };

        let now = Utc::now();
        let iat = now.timestamp();
        let exp = (now + Duration::days(7)).timestamp();

        Ok(encode(
            &Header::default(),
            &Self { iat, exp, user_id },
            &EncodingKey::from_secret(constants::CONFIG.jwt_secret.as_ref()),
        )?)
    }

    pub fn validate_token(token: &str) -> Result<Self, AppError> {
        // Ok(decode(
        //     token,
        //     &DecodingKey::from_secret(constants::CONFIG.jwt_secret.as_ref()),
        //     &Validation::default(),
        // )?
        // .claims)

        // TODO check exp is valid and user_id exist
        todo!();
    }
}

#[derive(Serialize)]
pub enum TokenType {
    Bearer,
}

#[derive(Serialize)]
pub struct Token {
    pub token: String,
    pub token_type: TokenType,
}
impl Token {
    pub fn new_bearer(token: String) -> Self {
        Self {
            token,
            token_type: TokenType::Bearer,
        }
    }
}
