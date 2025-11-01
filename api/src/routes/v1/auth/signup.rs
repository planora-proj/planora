use actix_web::{
    HttpResponse, Responder,
    cookie::{Cookie, SameSite},
    post, web,
};

use crate::{
    db::{models::User, repos::UserRepo},
    routes::common::{ApiError, ApiResult},
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

    let pool = manager.get_pool("planora").await.unwrap();

    // verify is any other user with the same email
    let user_repo = UserRepo::new(&pool);
    let u = user_repo
        .find_by_email(email.clone())
        .await
        .map_err(ApiError::from)?;

    if u.is_some() {
        return Ok(HttpResponse::NotAcceptable()
            .json(ApiResult::<()>::error("email is already registered")));
    }

    // create user
    let inserted_user = user_repo
        .create_user(&User {
            username,
            email: email.clone(),
            password: Some(password),
            ..Default::default()
        })
        .await
        .map_err(ApiError::from)?;
    tracing::info!("user has been created: {email}");

    // session creation
    let (access_token, _refresh_token) = jwt_service.generate_tokens(inserted_user.user_id)?;

    let cookie = Cookie::build(JwtService::JWT_SESSION_KEY, access_token)
        .path("/")
        .secure(false)
        .http_only(true)
        .same_site(SameSite::None)
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(ApiResult::<()>::success_message("signed up successfully")))
}
