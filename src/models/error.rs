use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use image::error::ImageError;
use log::error;
use s3::error::S3Error;
use serde::Serialize;
use std::{borrow::Borrow, error::Error as ErrorTrait};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("ALREADY_EXISTS")]
    AlreadyExists,
    #[error("BAD_REQUEST")]
    BadRequest,
    #[error("NOT_FOUND")]
    NotFound,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("FORBIDDEN")]
    Forbidden,
    #[error("INTERNAL_SERVER_ERROR")]
    InternalServerError { source: Box<dyn ErrorTrait> },
}

impl From<Box<dyn ErrorTrait>> for ApiError {
    fn from(v: Box<dyn ErrorTrait>) -> Self {
        Self::InternalServerError { source: v }
    }
}

// wrap [`ApiError`] into Result<_, ApiError> by default
impl<T> From<ApiError> for Result<T, ApiError> {
    fn from(error: ApiError) -> Self {
        Err(error)
    }
}

/// This is a helper struct we need in order to be able to return the internal Error from functions without building a http response first
#[derive(Serialize)]
struct ResponseBody {
    message: String,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let response_body = ResponseBody {
            message: format!("{}", self),
        };
        HttpResponse::build(self.status_code()).json(response_body)
    }

    /// Map http statuscodes to the corresponding [`Error`] variants
    fn status_code(&self) -> StatusCode {
        use ApiError::*;
        match self {
            AlreadyExists => StatusCode::CONFLICT,
            BadRequest => StatusCode::BAD_REQUEST,
            Forbidden => StatusCode::FORBIDDEN,
            InternalServerError { source: _ } => StatusCode::INTERNAL_SERVER_ERROR,
            NotFound => StatusCode::NOT_FOUND,
            Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }
}

impl From<S3Error> for ApiError {
    fn from(value: S3Error) -> Self {
        match value.borrow() {
            S3Error::Http(x) => match x.status_code() {
                StatusCode::NOT_FOUND => return ApiError::NotFound,
                _ => {
                    return ApiError::InternalServerError {
                        source: Box::new(value),
                    }
                }
            },
            _ => {
                error!("S3Error: {value}");
                Self::InternalServerError {
                    source: Box::new(value),
                }
            }
        }
    }
}

impl From<std::io::Error> for ApiError {
    fn from(value: std::io::Error) -> Self {
        Self::InternalServerError {
            source: Box::new(value),
        }
    }
}

impl From<ImageError> for ApiError {
    fn from(value: ImageError) -> Self {
        //TODO!
        todo!()
    }
}
