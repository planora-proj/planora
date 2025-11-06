use actix_web::{
    HttpRequest, HttpResponse, Responder,
    cookie::{Cookie, time},
    post, web,
};

use arx_gatehouse::{
    common::{ApiError, ApiResult},
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

#[cfg(test)]
mod tests {
    use actix_web::{App, test};

    use super::*;

    const JWT_SECRET: &'static str = "test_secret";
    const JWT_ACCESS_EXPIRY_MINUTES: i64 = 20;
    const JWT_REFRESH_EXPIRY_DAYS: i64 = 1;
    const USER_ID_1: uuid::Uuid = uuid::Uuid::from_u128(1);

    #[actix_web::test]
    async fn test_signout_success() {
        let jwt_service = JwtService::new(
            JWT_SECRET.to_string(),
            JWT_ACCESS_EXPIRY_MINUTES,
            JWT_REFRESH_EXPIRY_DAYS,
        );

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(jwt_service.clone()))
                .service(signout),
        )
        .await;

        let (access_token, _) = jwt_service
            .generate_tokens(USER_ID_1)
            .expect("failed to generate tokens");

        let req = test::TestRequest::post()
            .uri("/signout")
            .cookie(
                actix_web::cookie::Cookie::build(JwtService::JWT_SESSION_KEY, access_token.clone())
                    .http_only(true)
                    .secure(false)
                    .finish(),
            )
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        let body: ApiResult<()> = test::read_body_json(resp).await;
        assert!(body.success);
        assert_eq!(body.message, Some("logged out successfully".to_string()));
    }

    #[actix_web::test]
    async fn test_logout_without_cookie() {
        let jwt_service = JwtService::new(
            JWT_SECRET.to_string(),
            JWT_ACCESS_EXPIRY_MINUTES,
            JWT_REFRESH_EXPIRY_DAYS,
        );

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(jwt_service.clone()))
                .service(signout),
        )
        .await;

        let req = test::TestRequest::post().uri("/signout").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 401);

        let body: ApiResult<()> = test::read_body_json(resp).await;
        assert!(!body.success);
        assert_eq!(body.message, Some("user is not authenticated".to_string()));
    }
}
