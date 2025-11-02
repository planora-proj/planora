use actix_web::{HttpRequest, HttpResponse, Responder, get, web};

use crate::{
    db::repos::UserRepo,
    routes::common::{ApiError, ApiResult},
    services::{DbManager, JwtService},
};

#[derive(serde::Serialize)]
struct SafeUser {
    pub user_tag: Option<String>,
    pub username: String,
    pub email: String,
}

#[get("/profile")]
async fn profile(
    manager: web::Data<DbManager>,
    jwt_service: web::Data<JwtService>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    // extract the session token
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
    let safe_user = if let Some(user) = user_repo.find_by_userid(claims.sub).await? {
        tracing::info!(user = ?user);
        SafeUser {
            user_tag: user.user_tag,
            username: user.username,
            email: user.email,
        }
    } else {
        return Ok(
            HttpResponse::NotFound().json(ApiResult::<()>::success_message("user is not found"))
        );
    };

    Ok(HttpResponse::Ok().json(ApiResult::<SafeUser>::success(
        safe_user,
        "profile data".to_string(),
    )))
}
