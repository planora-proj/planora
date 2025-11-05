use actix_web::{HttpRequest, HttpResponse, Responder, get, web};

use super::helper::{extract_org_id, validate_org};
use crate::{
    db::{models::Project, repos::ProjectRepo},
    routes::common::{ApiError, ApiResult},
    services::DbManager,
};

#[get("/")]
async fn list_projects(
    manager: web::Data<DbManager>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let pool = manager.get_pool("planora").await.unwrap();

    let org_id = extract_org_id(&req).await?;
    validate_org(&pool, org_id).await?;

    tracing::trace!(%org_id, "Listing projects for organization");

    let project_repo = ProjectRepo::new(&pool);
    let projects = project_repo.find_by_orgid(org_id).await?;

    tracing::info!(%org_id, len = projects.len(), "Projects listed successfully");

    Ok(HttpResponse::Ok().json(ApiResult::<Vec<Project>>::success(
        projects,
        "projects".to_string(),
    )))
}
