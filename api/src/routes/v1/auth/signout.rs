use actix_web::{
    HttpRequest, HttpResponse, Responder,
    cookie::{Cookie, time},
    post, web,
};

use crate::{
    routes::common::{ApiError, ApiResult},
    services::JwtService,
};

#[post("/signout")]
async fn signout(
    req: HttpRequest,
    jwt_service: web::Data<JwtService>,
) -> Result<impl Responder, ApiError> {
    let token_cookie = if let Some(cookie) = req.cookie(JwtService::JWT_SESSION_KEY) {
        cookie
    } else {
        return Ok(
            HttpResponse::Unauthorized().json(ApiResult::<()>::error("user is not authenticated"))
        );
    };

    // verify the token
    jwt_service.verify_token(token_cookie.value())?;

    let expired_cookie = Cookie::build(JwtService::JWT_SESSION_KEY, "")
        .path("/")
        .secure(false)
        .http_only(true)
        .max_age(time::Duration::seconds(-1))
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(expired_cookie)
        .json(ApiResult::<()>::success_message("logged out successfully")))
}
