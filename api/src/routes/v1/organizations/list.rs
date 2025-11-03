use actix_web::{HttpRequest, HttpResponse, Responder, get, web};

use crate::{
    db::{
        models::Organization,
        repos::{OrgRepo, UserRepo},
    },
    routes::common::{ApiError, ApiResult},
    services::{DbManager, JwtService},
};

#[get("/")]
async fn list_organizations(
    manager: web::Data<DbManager>,
    jwt_service: web::Data<JwtService>,
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

    Ok(
        HttpResponse::Ok().json(ApiResult::<Vec<Organization>>::success(
            orgs,
            Some("organization has been creatd".to_string()),
        )),
    )
}
