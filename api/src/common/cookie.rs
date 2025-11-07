use actix_web::{HttpRequest, dev::ServiceRequest};

use super::ApiError;
use crate::services::auth::constants::{JWT_ACCESS_TOKEN_KEY, JWT_REFRESH_TOKEN_KEY};

pub fn extract_access_token(req: &ServiceRequest) -> Result<String, ApiError> {
    tracing::trace!("Extracting JWT Access Token from cookies");

    let access_cookie = req
        .cookie(JWT_ACCESS_TOKEN_KEY)
        .ok_or_else(|| ApiError::Unauthorized("Missing authentication token".to_string()))?;

    let value = access_cookie.value().to_owned();

    tracing::trace!(%value, "Extracted JWT Access Token");

    Ok(value)
}

pub fn extract_refresh_token(req: &HttpRequest) -> Result<String, ApiError> {
    tracing::trace!("Extracting JWT Refresh Token from cookies");

    let access_cookie = req
        .cookie(JWT_REFRESH_TOKEN_KEY)
        .ok_or_else(|| ApiError::Unauthorized("Missing authentication token".to_string()))?;

    let value = access_cookie.value().to_owned();

    tracing::trace!(%value, "Extracted JWT Refresh Token");

    Ok(value)
}
