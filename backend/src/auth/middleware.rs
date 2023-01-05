use actix_web::FromRequest;
use futures::future::{err, ok, Ready};

use crate::errors::AppError;

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
            Some(v) => v.to_str().unwrap(),
            None => return err(AppError::server_error("Authorization header missing")),
        };

        let mut split_token = token.split_whitespace();
        match split_token.next() {
            Some(token_type) => {
                if token_type != "Bearer" {
                    return err(AppError::server_error("Invalid token type"));
                }
            }
            None => return err(AppError::server_error("Token type missing")),
        };

        let claims = match split_token.next() {
            Some(token) => match Claims::validate_token(token) {
                Ok(v) => v,
                Err(_) => return err(AppError::server_error("Token invalid")),
            },
            None => return err(AppError::server_error("Token missing")),
        };

        ok(Self { claims })
    }
}
