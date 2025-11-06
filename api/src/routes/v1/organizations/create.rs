use actix_web::{HttpRequest, HttpResponse, Responder, post, web};

use arx_gatehouse::{
    common::{ApiError, ApiResult, headers::extract_user_id},
    db::{models::Organization, repos::OrgRepo},
    services::DbManager,
};

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(serde::Deserialize)]
struct CreateOrg {
    pub name: String,
    pub subdomain: String,
}

#[post("")]
async fn create_organization(
    manager: web::Data<DbManager>,
    payload: web::Json<CreateOrg>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let org_name = payload.name.clone();
    let org_domain = payload.subdomain.clone();

    let user_id = extract_user_id(&req)?;

    tracing::trace!(%user_id, "create organization");

    let pool = manager.get_planora_pool().await?;
    let org_repo = OrgRepo::new(&pool);

    let inserted_org = org_repo
        .create_org(&Organization {
            owner_id: user_id,
            name: org_name,
            subdomain: org_domain,
            ..Default::default()
        })
        .await?;

    tracing::info!(%user_id, "created organization");

    Ok(HttpResponse::Ok().json(ApiResult::<Organization>::success(
        inserted_org,
        Some("organization has been created".to_string()),
    )))
}
