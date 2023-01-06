use actix_web::FromRequest;
use futures::future::{err, ok, Ready};

use crate::{constants, errors::AppError};

use super::auth_token::Claims;

pub struct AuthService {
    pub claims: Claims,
}

impl FromRequest for AuthService {
    type Error = AppError;
    type Future = Ready<Result<Self, AppError>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        let token = match auth_header {
            Some(v) => match v.to_str() {
                Ok(v) => v,
                Err(_) => return err(AppError::unexpected_error()),
            },
            None => {
                return err(AppError::unauthorized_error(
                    constants::MESSAGE_AUTHORIZATION_HEADER_MISSING,
                ))
            }
        };

        let mut split_token = token.split_whitespace();
        match split_token.next() {
            Some(token_type) => {
                if token_type != "Bearer" {
                    return err(AppError::unauthorized_error(
                        constants::MESSAGE_INVALID_TOKEN_TYPE,
                    ));
                }
            }
            None => {
                return err(AppError::unauthorized_error(
                    constants::MESSAGE_TOKEN_TYPE_MISSING,
                ))
            }
        };

        let claims = match split_token.next() {
            Some(token) => match Claims::validate_token(token) {
                Ok(v) => v,
                Err(e) => return err(e),
            },
            None => {
                return err(AppError::unauthorized_error(
                    constants::MESSAGE_TOKEN_MISSING,
                ))
            }
        };

        ok(Self { claims })
    }
}
