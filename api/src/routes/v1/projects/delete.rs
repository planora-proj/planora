use actix_web::{HttpRequest, Responder, delete, web};

use super::helper::validate_org;
use arx_gatehouse::{
    common::{ApiError, ApiResult, headers::extract_org_id},
    db::repos::ProjectRepo,
    services::DbManager,
};

#[derive(serde::Deserialize)]
pub(crate) struct DeleteProject {
    project_id: uuid::Uuid,
}

#[delete("")]
async fn delete_project(
    manager: web::Data<DbManager>,
    payload: web::Json<DeleteProject>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let project_id = payload.project_id.clone();

    let pool = manager.get_planora_pool().await?;

    let org_id = extract_org_id(&req)?;
    validate_org(&pool, org_id).await?;

    tracing::trace!(%project_id, %org_id, "delete project");

    let project_repo = ProjectRepo::new(&pool);

    let affected_rows = project_repo
        .delete_by_projectid(project_id, org_id)
        .await
        .map_err(|e| {
            tracing::error!(error = ?e, %project_id, %org_id, "Failed to delete project");
            ApiError::Internal("Error deleting project".into())
        })?;

    if affected_rows == 0 {
        tracing::warn!(%project_id, %org_id, "No project found to delete");
        return ApiResult::to_not_found("Project not found");
    }

    tracing::info!(%project_id, %org_id, %affected_rows, "Project deleted successfully");

    ApiResult::to_ok_response("Project has been deleted successfully", affected_rows)
}
