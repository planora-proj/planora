use actix_web::{HttpRequest, HttpResponse, Responder, post, web};

use crate::{
    db::{
        models::Organization,
        repos::{OrgRepo, UserRepo},
    },
    routes::common::{ApiError, ApiResult},
    services::{DbManager, JwtService},
};

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(serde::Deserialize)]
struct CreateOrg {
    pub name: String,
    pub subdomain: String,
}

#[post("/")]
async fn create_organization(
    manager: web::Data<DbManager>,
    jwt_service: web::Data<JwtService>,
    payload: web::Json<CreateOrg>,
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

    let org_name = payload.name.clone();
    let org_domain = payload.subdomain.clone();

    let org_repo = OrgRepo::new(&pool);
    let inserted_org = org_repo
        .create_org(&Organization {
            owner_id: user.user_id,
            name: org_name,
            subdomain: org_domain,
            ..Default::default()
        })
        .await?;
    tracing::debug!("created organization for {:?}", user);

    Ok(HttpResponse::Ok().json(ApiResult::<Organization>::success(
        inserted_org,
        Some("organization has been creatd".to_string()),
    )))
}
