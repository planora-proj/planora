use actix_web::{HttpRequest, HttpResponse, Responder, delete, web};

use arx_gatehouse::{
    common::{
        ApiError, ApiResult,
        headers::{extract_org_id, extract_user_id},
    },
    db::repos::OrgRepo,
    services::DbManager,
};

#[delete("")]
async fn delete_organization(
    manager: web::Data<DbManager>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let user_id = extract_user_id(&req)?;
    let org_id = extract_org_id(&req)?;

    tracing::trace!(%user_id, %org_id, "delete organization");

    let pool = manager.get_planora_pool().await?;
    let org_repo = OrgRepo::new(&pool);

    match org_repo.find_by_orgid(org_id).await? {
        Some(org) if org.owner_id == user_id => {
            tracing::trace!(%user_id, %org_id, "organization has been found");
        }
        _ => {
            tracing::error!(%user_id, %org_id, "failed to retrieve organization for the user");
            return Ok(HttpResponse::NotFound().json(ApiResult::<()>::error(
                "organization is not found for the user",
            )));
        }
    };

    let affected_row = org_repo.delete_by_orgid(org_id).await?;

    tracing::info!(%user_id, %org_id, %affected_row, "organization deleted");

    Ok(HttpResponse::Ok().json(ApiResult::<u64>::success(
        affected_row,
        Some("organization has been deleted".to_string()),
    )))
}
