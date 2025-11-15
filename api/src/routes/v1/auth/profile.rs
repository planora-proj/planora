use actix_web::{HttpRequest, Responder, get, web};

use arx_gatehouse::common::{ApiError, ApiResult, headers::extract_user_id};
use arx_gatehouse::db::{dto::user::SafeUser, repos::UserRepo};
use arx_gatehouse::services::DbManager;

#[get("/profile")]
async fn profile(
    manager: web::Data<DbManager>,
    req: HttpRequest,
) -> Result<impl Responder, ApiError> {
    let user_id = extract_user_id(&req)?;
    tracing::trace!(%user_id, "request for profile data");

    let pool = manager.get_planora_pool().await?;
    let user_repo = UserRepo::new(&pool);

    let user = match user_repo.find_by_userid(user_id).await? {
        Some(user) => {
            tracing::trace!(%user_id, "user has been found");
            user
        }
        None => {
            tracing::error!(%user_id, "failed to retrieve user");
            return ApiResult::to_not_found("user is not found");
        }
    };

    let safe_user = SafeUser {
        user_tag: user.user_tag,
        username: user.username,
        email: user.email,
    };

    tracing::info!(%user_id, "sending profile data");

    ApiResult::to_ok_response("profile data", safe_user)
}
