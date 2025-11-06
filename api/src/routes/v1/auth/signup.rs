use actix_web::{
    HttpResponse, Responder,
    cookie::{Cookie, SameSite},
    post, web,
};

use arx_gatehouse::{
    common::{ApiError, ApiResult},
    db::{models::User, repos::UserRepo},
    services::{DbManager, JwtService},
};

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(serde::Deserialize)]
struct SignupPayload {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[post("/signup")]
async fn signup(
    manager: web::Data<DbManager>,
    jwt_service: web::Data<JwtService>,
    payload: web::Json<SignupPayload>,
) -> Result<impl Responder, ApiError> {
    let username = payload.username.clone();
    let email = payload.email.clone();
    let password = payload.password.clone();

    tracing::trace!(%email, "signing up");

    let pool = manager.get_planora_pool().await?;

    let user_repo = UserRepo::new(&pool);
    match user_repo.find_by_email(email.clone()).await? {
        Some(_) => {
            tracing::error!(%email, "email is already registered");
            return Ok(HttpResponse::Conflict()
                .json(ApiResult::<()>::error("email is already registered")));
        }
        _ => {}
    };

    tracing::trace!(%email, "creating a user");

    let inserted_user = user_repo
        .create_user(&User {
            username,
            email: email.clone(),
            password: Some(password),
            ..Default::default()
        })
        .await?;

    tracing::info!(%email, "user has been created successfuly");

    tracing::trace!(%email, "generating session token");
    let (access_token, _refresh_token) = jwt_service.generate_tokens(inserted_user.user_id)?;

    let cookie = Cookie::build(JwtService::JWT_SESSION_KEY, access_token)
        .path("/")
        .secure(false)
        .http_only(true)
        .same_site(SameSite::None)
        .finish();

    tracing::info!("user has been signed up successfully: {email}");

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(ApiResult::<()>::success_message("signed up successfully")))
}
