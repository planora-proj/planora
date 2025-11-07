use actix_web::{HttpResponse, ResponseError};

use super::ApiResult;
use crate::services::auth::AuthError;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ApiError {
    #[error("Database connection failed: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Authentication failed")]
    AuthError(#[from] AuthError),

    #[error("Header value to string conversion error: {0}")]
    ToStrError(#[from] actix_web::http::header::ToStrError),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Internal server error: {0}")]
    Internal(String),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        use ApiError::*;

        match self {
            DatabaseError(err) => {
                tracing::error!(error = %err);
                HttpResponse::InternalServerError()
                    .json(ApiResult::<()>::error("Internal server error"))
            }
            AuthError(err) => {
                tracing::error!(error = %err);
                HttpResponse::InternalServerError()
                    .json(ApiResult::<()>::error("Internal server error"))
            }
            ToStrError(err) => {
                tracing::error!(error = %err);
                HttpResponse::InternalServerError()
                    .json(ApiResult::<()>::error("Conversion failed"))
            }
            ApiError::BadRequest(msg) => {
                HttpResponse::BadRequest().json(ApiResult::<()>::error(msg))
            }
            ApiError::Unauthorized(msg) => {
                HttpResponse::Unauthorized().json(ApiResult::<()>::error(msg))
            }
            ApiError::Internal(msg) => {
                HttpResponse::InternalServerError().json(ApiResult::<()>::error(msg))
            }
        }
    }
}
