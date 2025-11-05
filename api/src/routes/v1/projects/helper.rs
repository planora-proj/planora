use actix_web::HttpRequest;

use crate::{db::repos::OrgRepo, routes::common::ApiError};

pub async fn extract_org_id(req: &HttpRequest) -> Result<uuid::Uuid, ApiError> {
    tracing::trace!("Extracting organization ID from request headers");

    let header = req.headers().get("X-Organization-Id").ok_or_else(|| {
        tracing::warn!("Missing X-Organization-Id header");
        ApiError::BadRequest("Missing X-Organization-Id header".into())
    })?;

    let value = header.to_str().map_err(|_| {
        tracing::warn!("Invalid X-Organization-Id header encoding");
        ApiError::BadRequest("Invalid X-Organization-Id header encoding".into())
    })?;

    let org_id = uuid::Uuid::parse_str(value).map_err(|_| {
        tracing::warn!(header = %value, "Invalid organization UUID format");
        ApiError::BadRequest("Invalid organization UUID format".into())
    })?;

    tracing::trace!(%org_id, "Extracted organization ID");
    Ok(org_id)
}

pub async fn validate_org(pool: &sqlx::PgPool, org_id: uuid::Uuid) -> Result<(), ApiError> {
    let repo = OrgRepo::new(pool);
    tracing::trace!(%org_id, "Validating organization existence");

    match repo.find_by_orgid(org_id).await {
        Ok(Some(_)) => {
            tracing::info!(%org_id, "Organization verified");
            Ok(())
        }
        Ok(None) => {
            tracing::warn!(%org_id, "Unauthorized access: organization not found");
            Err(ApiError::Unauthorized(
                "You are not allowed to perform this action".into(),
            ))
        }
        Err(e) => {
            tracing::error!(error = ?e, %org_id, "Database error while validating org");
            Err(ApiError::Internal("Failed to verify organization".into()))
        }
    }
}
