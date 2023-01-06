use std::pin::Pin;

use actix_web::{web::Data, FromRequest};
use futures::Future;

use crate::{constants, errors::AppError, repository::mongodb_repos::MongoRepo};

use super::auth_token::Claims;

pub struct AuthService {
    pub claims: Claims,
}

impl FromRequest for AuthService {
    type Error = AppError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, AppError>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            let auth_header = req.headers().get("Authorization");
            let token = match auth_header {
                Some(v) => match v.to_str() {
                    Ok(v) => v,
                    Err(_) => return Err(AppError::unexpected_error()),
                },
                None => {
                    return Err(AppError::unauthorized_error(
                        constants::MESSAGE_AUTHORIZATION_HEADER_MISSING,
                    ))
                }
            };

            let mut split_token = token.split_whitespace();
            match split_token.next() {
                Some(token_type) => {
                    if token_type != "Bearer" {
                        return Err(AppError::unauthorized_error(
                            constants::MESSAGE_INVALID_TOKEN_TYPE,
                        ));
                    }
                }
                None => {
                    return Err(AppError::unauthorized_error(
                        constants::MESSAGE_TOKEN_TYPE_MISSING,
                    ))
                }
            };

            let claims = match split_token.next() {
                Some(token) => {
                    let db = match req.app_data::<Data<MongoRepo>>() {
                        Some(v) => v,
                        None => return Err(AppError::unexpected_error()),
                    };
                    match Claims::validate_token(token, db).await {
                        Ok(v) => v,
                        Err(e) => return Err(e),
                    }
                }
                None => {
                    return Err(AppError::unauthorized_error(
                        constants::MESSAGE_TOKEN_MISSING,
                    ))
                }
            };

            Ok(Self { claims })
        })
    }
}
