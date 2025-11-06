use actix_web::HttpRequest;

use super::{
    ApiError,
    constants::{X_ORG_ID_HEADER, X_USER_ID_HEADER},
};

pub fn extract_user_id(req: &HttpRequest) -> Result<uuid::Uuid, ApiError> {
    tracing::trace!("Extracting user ID from request headers");

    let header = req.headers().get(X_USER_ID_HEADER).ok_or_else(|| {
        tracing::warn!("Missing X-User-Id header");
        ApiError::BadRequest("Missing X-User-Id header".into())
    })?;

    let value = header.to_str().map_err(|_| {
        tracing::warn!("Invalid X-User-Id header encoding");
        ApiError::BadRequest("Invalid X-User-Id header".into())
    })?;

    let user_id = uuid::Uuid::parse_str(value).map_err(|_| {
        tracing::warn!(header = %value, "Invalid user UUID format");
        ApiError::BadRequest("Invalid X-User-Id header".into())
    })?;

    tracing::trace!(%user_id, "Extracted user ID");
    Ok(user_id)
}

pub fn extract_org_id(req: &HttpRequest) -> Result<uuid::Uuid, ApiError> {
    tracing::trace!("Extracting organization ID from request headers");

    let header = req.headers().get(X_ORG_ID_HEADER).ok_or_else(|| {
        tracing::warn!("Missing X-Organization-Id header");
        ApiError::BadRequest("Missing X-Organization-Id header".into())
    })?;

    let value = header.to_str().map_err(|_| {
        tracing::warn!("Invalid X-Organization-Id header encoding");
        ApiError::BadRequest("Invalid X-Organization-Id header".into())
    })?;

    let org_id = uuid::Uuid::parse_str(value).map_err(|_| {
        tracing::warn!(header = %value, "Invalid organization UUID format");
        ApiError::BadRequest("Invalid X-Organization-Id header".into())
    })?;

    tracing::trace!(%org_id, "Extracted organization ID");
    Ok(org_id)
}
