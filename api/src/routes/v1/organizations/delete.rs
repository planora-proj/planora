use actix_web::{HttpRequest, HttpResponse, Responder, delete, web};

use crate::{
    db::repos::{OrgRepo, UserRepo},
    routes::common::{ApiError, ApiResult},
    services::{DbManager, JwtService},
};

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(serde::Deserialize)]
struct DeleteOrg {
    pub org_id: Option<uuid::Uuid>,
    pub subdomain: Option<String>,
}

#[delete("/")]
async fn delete_organization(
    manager: web::Data<DbManager>,
    jwt_service: web::Data<JwtService>,
    payload: web::Json<DeleteOrg>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    // TODO: extract this user authentication to a middleware
    let token_cookie = if let Some(cookie) = req.cookie(JwtService::JWT_SESSION_KEY) {
        cookie
    } else {
        return Ok(
            HttpResponse::Unauthorized().json(ApiResult::<()>::error("authentication required"))
        );
    };
    let pool = manager.get_pool("planora").await.unwrap();
    let claims = jwt_service.verify_token(token_cookie.value())?;
    let user_repo = UserRepo::new(&pool);

    let user = if let Some(user) = user_repo.find_by_userid(claims.sub).await? {
        user
    } else {
        return Ok(
            HttpResponse::Unauthorized().json(ApiResult::<()>::error("authentication required"))
        );
    };

    let org_repo = OrgRepo::new(&pool);
    let orgs = org_repo.find_by_ownerid(user.user_id).await?;
    let org_ids = orgs.iter().map(|v| v.organization_id).collect::<Vec<_>>();
    let org_subdomains = orgs.iter().map(|v| v.subdomain.clone()).collect::<Vec<_>>();

    // checking the payload
    let affected_row = match payload.0 {
        DeleteOrg {
            org_id: Some(org_id),
            ..
        } => {
            if !org_ids.contains(&org_id) {
                return Ok(HttpResponse::Unauthorized().json(ApiResult::<()>::error(
                    "you don't have privilege to delete the organization",
                )));
            }

            tracing::trace!("deleted the organization: {}", org_id);
            org_repo.delete_by_orgid(org_id).await?
        }
        DeleteOrg {
            subdomain: Some(subdomain),
            ..
        } => {
            if !org_subdomains.contains(&subdomain) {
                return Ok(HttpResponse::Unauthorized().json(ApiResult::<()>::error(
                    "you don't have privilege to delete the organization",
                )));
            }
            tracing::trace!("deleted the organization: {}", subdomain);
            org_repo.delete_by_subdomain(subdomain).await?
        }
        _ => {
            return Ok(HttpResponse::BadRequest().json(ApiResult::<()>::error("invalid request")));
        }
    };

    Ok(HttpResponse::Ok().json(ApiResult::<u64>::success(
        affected_row,
        Some("organization has been deleted".to_string()),
    )))
}
