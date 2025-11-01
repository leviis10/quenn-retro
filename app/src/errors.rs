use crate::dtos::global::ErrorResponse;
use axum::http::StatusCode;
use axum::http::header::ToStrError;
use axum::response::{IntoResponse, Response};
use sea_orm::DbErr;
use serde::Serialize;
use std::backtrace::Backtrace;
use std::convert::Infallible;

#[derive(Serialize)]
pub enum ErrorCode {
    Database,
    NotFound,
    Unauthorized,
    Parse,
    Forbidden,
    Infallible,
}

pub enum AppError {
    Database(DbErr),
    NotFound(String, Backtrace),
    Unauthorized(String),
    ParseHeader(ToStrError),
    ParseQuery(serde_qs::Error),
    ParseUuid(uuid::Error),
    WrongSecret(String, Backtrace),
    Infallible(Infallible),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, response) = match self {
            AppError::Database(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error_code: ErrorCode::Database,
                    message: err.to_string(),
                },
            ),
            AppError::NotFound(err, backtrace) => {
                tracing::error!("{backtrace:#?}");
                (
                    StatusCode::NOT_FOUND,
                    ErrorResponse {
                        error_code: ErrorCode::NotFound,
                        message: err,
                    },
                )
            }
            AppError::Unauthorized(err) => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    error_code: ErrorCode::Unauthorized,
                    message: err,
                },
            ),
            AppError::ParseHeader(err) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    error_code: ErrorCode::Parse,
                    message: err.to_string(),
                },
            ),
            AppError::ParseQuery(err) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    error_code: ErrorCode::Parse,
                    message: err.to_string(),
                },
            ),
            AppError::WrongSecret(err, backtrace) => {
                tracing::error!("{backtrace:#?}");
                (
                    StatusCode::FORBIDDEN,
                    ErrorResponse {
                        error_code: ErrorCode::Forbidden,
                        message: err,
                    },
                )
            }
            AppError::Infallible(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error_code: ErrorCode::Infallible,
                    message: err.to_string(),
                },
            ),
            AppError::ParseUuid(err) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    error_code: ErrorCode::Parse,
                    message: err.to_string(),
                },
            ),
        };

        (status, response).into_response()
    }
}

impl From<DbErr> for AppError {
    fn from(error: DbErr) -> Self {
        AppError::Database(error)
    }
}

impl From<ToStrError> for AppError {
    fn from(error: ToStrError) -> Self {
        AppError::ParseHeader(error)
    }
}

impl From<serde_qs::Error> for AppError {
    fn from(error: serde_qs::Error) -> Self {
        AppError::ParseQuery(error)
    }
}

impl From<Infallible> for AppError {
    fn from(error: Infallible) -> Self {
        AppError::Infallible(error)
    }
}

impl From<uuid::Error> for AppError {
    fn from(error: uuid::Error) -> Self {
        AppError::ParseUuid(error)
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
