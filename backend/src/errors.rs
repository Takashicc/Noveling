use core::fmt;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

#[derive(Debug)]
pub enum AppErrorType {
    Db,
    AlreadyExists,
    Server,
}

#[derive(Debug)]
pub struct AppError {
    pub error_type: AppErrorType,
    pub status_code: Option<StatusCode>,
    pub message: Option<String>,
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub message: String,
}

impl AppError {
    pub fn message(&self) -> String {
        if let Some(message) = &self.message {
            return message.to_owned();
        }

        "An unexpected error has occurred".to_string()
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::Db => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::AlreadyExists => StatusCode::BAD_REQUEST,
            AppErrorType::Server => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
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
