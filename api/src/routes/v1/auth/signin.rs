use actix_web::{HttpResponse, Responder, post, web};

use arx_gatehouse::{
    common::{ApiError, ApiResult},
    db::repos::UserRepo,
    services::{DbManager, JwtService, auth::cookie::build_cookie},
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

    tracing::trace!(%email, "signing in");

    let pool = manager.get_planora_pool().await?;
    let user_repo = UserRepo::new(&pool);

    let user = match user_repo.find_by_email(email.clone()).await? {
        Some(user) => {
            tracing::trace!(%email, "user has been found");
            user
        }
        None => {
            tracing::error!(%email, "invalid email");
            return Ok(HttpResponse::NotFound().json(ApiResult::<()>::error("invalid email")));
        }
    };

    match user.password {
        Some(pass) if pass == password => {}
        _ => {
            return Ok(
                HttpResponse::Unauthorized().json(ApiResult::<()>::error("invalid credentials"))
            );
        }
    }

    tracing::trace!(%email, "valid user credentials");

    tracing::trace!(%email, "generate session token");
    let (access_token, refresh_token) = jwt_service.generate_tokens(user.user_id)?;
    let (access_token_cookie, refresh_token_cookie) = build_cookie(access_token, refresh_token);

    tracing::info!(%email, "signed in successfully");

    Ok(HttpResponse::Ok()
        .cookie(access_token_cookie)
        .cookie(refresh_token_cookie)
        .json(ApiResult::<()>::success_message("signed in successfully")))
}
