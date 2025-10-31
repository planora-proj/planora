use actix_web::{HttpResponse, ResponseError};

use super::ApiResult;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ApiError {
    #[error("Database connection failed: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("jwt service failed")]
    JwtError(#[from] jsonwebtoken::errors::Error),
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
            JwtError(err) => {
                tracing::error!(error = %err);
                HttpResponse::InternalServerError()
                    .json(ApiResult::<()>::error("Internal server error"))
            }
        }
    }
}
