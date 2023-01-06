use actix_web::web::Data;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::{constants, errors::AppError, repository::mongodb_repos::MongoRepo};

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
            None => return Err(AppError::unexpected_error()),
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

    pub async fn validate_token(token: &str, db: &Data<MongoRepo>) -> Result<Self, AppError> {
        let claims: Self = match decode(
            token,
            &DecodingKey::from_secret(constants::CONFIG.jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(v) => v.claims,
            Err(_) => return Err(AppError::unauthorized_error("Token invalid")),
        };

        let now = Utc::now().timestamp();
        if now >= claims.exp {
            return Err(AppError::unauthorized_error("Token expired"));
        }

        let user_exits = db.user_exists_by_id(&claims.user_id).await?;
        if !user_exits {
            return Err(AppError::unauthorized_error("Cannot find user from token"));
        }

        Ok(claims)
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
