use core::fmt;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

use crate::constants;

#[derive(Debug)]
pub enum AppErrorType {
    Db,
    Server,
    User,
}

#[derive(Debug)]
pub struct AppError {
    error_type: AppErrorType,
    status_code: Option<StatusCode>,
    message: Option<String>,
}

impl AppError {
    pub fn bad_request_error(message: &str) -> Self {
        Self {
            error_type: AppErrorType::User,
            status_code: Some(StatusCode::BAD_REQUEST),
            message: Some(message.to_string()),
        }
    }

    pub fn unauthorized_error(message: &str) -> Self {
        Self {
            error_type: AppErrorType::User,
            status_code: Some(StatusCode::UNAUTHORIZED),
            message: Some(message.to_string()),
        }
    }

    pub fn unexpected_error() -> Self {
        Self {
            error_type: AppErrorType::Server,
            status_code: Some(StatusCode::INTERNAL_SERVER_ERROR),
            message: Some(constants::MESSAGE_UNEXPECTED_ERROR.to_string()),
        }
    }

    fn message(&self) -> String {
        if let Some(message) = &self.message {
            return message.to_owned();
        }

        constants::MESSAGE_UNEXPECTED_ERROR.to_string()
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    message: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        if let Some(code) = self.status_code {
            return code;
        }

        match self.error_type {
            AppErrorType::Db => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::Server => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::User => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let status_code = self.status_code();

        HttpResponse::build(status_code).json(AppErrorResponse {
            message: self.message(),
        })
    }
}

impl From<mongodb::error::Error> for AppError {
    fn from(err: mongodb::error::Error) -> Self {
        println!("{}", err); // TODO Use log

        Self {
            error_type: AppErrorType::Db,
            status_code: None,
            message: None,
        }
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(_: bcrypt::BcryptError) -> Self {
        Self {
            error_type: AppErrorType::Server,
            status_code: None,
            message: None,
        }
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(_: jsonwebtoken::errors::Error) -> Self {
        Self {
            error_type: AppErrorType::Server,
            status_code: None,
            message: None,
        }
    }
}

impl From<mongodb::bson::oid::Error> for AppError {
    fn from(_: mongodb::bson::oid::Error) -> Self {
        Self::unexpected_error()
    }
}
