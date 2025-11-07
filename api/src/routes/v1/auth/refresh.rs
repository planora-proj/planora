use actix_web::{HttpRequest, HttpResponse, Responder, get, web};

use arx_gatehouse::{
    common::{ApiError, ApiResult, cookie::extract_refresh_token},
    services::{AuthService, auth::cookie::build_cookie_cn},
};

#[get("/refresh")]
async fn refresh(
    req: HttpRequest,
    auth_service: web::Data<AuthService>,
) -> Result<impl Responder, ApiError> {
    tracing::trace!("refreshing jwt token");

    let refresh_token = extract_refresh_token(&req)?;

    let access_token = auth_service.jwt_generate_access_token(refresh_token)?;

    let user_id = auth_service.jwt_verify_access_token(&access_token)?;

    let access_token_cookie = build_cookie_cn(true, access_token);

    tracing::trace!(%user_id, "jwt access token refreshed");

    Ok(HttpResponse::Ok()
        .cookie(access_token_cookie)
        .json(ApiResult::ok("access token generated")))
}
