use actix_web::{HttpRequest, HttpResponse, Responder, delete, web};

use super::helper::{extract_org_id, validate_org};
use crate::{
    db::repos::ProjectRepo,
    routes::common::{ApiError, ApiResult},
    services::DbManager,
};

#[derive(serde::Deserialize)]
pub(crate) struct DeleteProject {
    project_id: uuid::Uuid,
}

#[delete("/")]
async fn delete_project(
    manager: web::Data<DbManager>,
    payload: web::Json<DeleteProject>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let project_id = payload.project_id.clone();
    tracing::trace!(%project_id, "Received request to delete project");

    let pool = manager.get_pool("planora").await.unwrap();

    let org_id = extract_org_id(&req).await?;
    validate_org(&pool, org_id).await?;

    tracing::trace!(%project_id, %org_id, "Deleting project for organization");

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
        return Ok(
            HttpResponse::NotFound().json(ApiResult::<()>::error("Project not found".to_string()))
        );
    }

    tracing::info!(%project_id, %org_id, %affected_rows, "Project deleted successfully");

    Ok(HttpResponse::Ok().json(ApiResult::<u64>::success(
        affected_rows,
        "Project has been deleted successfully".to_string(),
    )))
}
