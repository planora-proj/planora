use actix_web::{
    HttpResponse, Responder,
    cookie::{Cookie, SameSite},
    post, web,
};

use crate::{
    db::repos::UserRepo,
    routes::common::{ApiError, ApiResult},
    services::{DbManager, JwtService},
};

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(serde::Deserialize)]
struct SigninPayload {
    pub email: String,
    pub password: String,
}

#[post("/signin")]
async fn signin(
    manager: web::Data<DbManager>,
    jwt_service: web::Data<JwtService>,
    payload: web::Json<SigninPayload>,
) -> Result<impl Responder, ApiError> {
    let email = payload.email.clone();
    let password = payload.password.clone();

    let pool = manager.get_pool("planora").await.unwrap();
    let user_repo = UserRepo::new(&pool);

    let u = user_repo
        .find_by_email(email.clone())
        .await
        .map_err(ApiError::from)?;

    if u.is_none() {
        return Ok(
            HttpResponse::NotAcceptable().json(ApiResult::<()>::error("email is not registered"))
        );
    }

    let u = u.unwrap();
    if u.password.is_some() && password != u.password.unwrap() {
        return Ok(HttpResponse::Unauthorized().json(ApiResult::<()>::error("invalid credentials")));
    }
    tracing::info!("user has been signed in successfully: {email}");

    // session creation
    let (access_token, _refresh_token) = jwt_service.generate_tokens(u.user_id)?;

    let cookie = Cookie::build(JwtService::JWT_SESSION_KEY, access_token)
        .path("/")
        .secure(false)
        .http_only(true)
        .same_site(SameSite::None)
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(ApiResult::<()>::success_message("signed in successfully")))
}
