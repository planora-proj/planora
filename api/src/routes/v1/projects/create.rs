use actix_web::{HttpRequest, HttpResponse, Responder, post, web};

use super::helper::{extract_org_id, validate_org};

use arx_gatehouse::{
    common::{ApiError, ApiResult},
    db::{models::Project, repos::ProjectRepo},
    services::DbManager,
};

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(serde::Deserialize)]
struct CreateProject {
    pub name: String,
    pub description: Option<String>,
}

#[post("/")]
async fn create_project(
    manager: web::Data<DbManager>,
    payload: web::Json<CreateProject>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let name = payload.name.clone();
    let description = payload.description.clone();

    let pool = manager.get_pool("planora").await.unwrap();

    let org_id = extract_org_id(&req).await?;
    validate_org(&pool, org_id).await?;

    tracing::trace!(%name, %org_id, "Creating project for organization");

    let project_repo = ProjectRepo::new(&pool);

    let inserted_project = project_repo
        .create_project(
            &Project {
                organization_id: org_id,
                name,
                description,
                ..Default::default()
            },
            org_id,
        )
        .await?;

    tracing::info!(%inserted_project.project_id, %org_id, "Project created successfully");

    Ok(HttpResponse::Ok().json(ApiResult::<Project>::success(
        inserted_project,
        "project has been successfully created".to_string(),
    )))
}
