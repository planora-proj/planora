use actix_web::{HttpResponse, Responder, get, web};

use arx_gatehouse::{
    common::{ApiError, ApiResult, PaginatedResult, PaginationQuery},
    db::repos::UserRepo,
    services::DbManager,
};

const DEFAULT_PER_PAGE: u64 = 20;
const MIN_DEFAULT_PER_PAGE: u64 = 10;
const MAX_DEFAULT_PER_PAGE: u64 = 50;
const SECRET_KEY: &'static str = "arx";

#[derive(serde::Deserialize)]
struct Payload {
    secret: Option<String>,
}

#[get("/users")]
async fn get_users(
    manager: web::Data<DbManager>,
    payload: web::Query<PaginationQuery>,
    secret: web::Query<Payload>,
) -> Result<impl Responder, ApiError> {
    if secret.secret.as_deref() != Some(SECRET_KEY) {
        return ApiResult::to_unauthorized("not authorized to perform this action");
    }
    let pool = manager.get_pool("planora").await.unwrap();

    let per_page = payload.per_page.unwrap_or(DEFAULT_PER_PAGE);
    if per_page < MIN_DEFAULT_PER_PAGE || per_page > MAX_DEFAULT_PER_PAGE {
        return ApiResult::to_bad_request("per_page parameter is out of bounds");
    }

    let offset = payload
        .page
        .map(|page| (page.saturating_sub(1)) * per_page)
        .unwrap_or(0);

    let user_repo = UserRepo::new(&pool);
    let users = user_repo.list_users(per_page, offset).await?;
    let len = users.len() as u64;

    tracing::debug!(len = ?len, offset = ?offset);

    Ok(HttpResponse::Ok().json(PaginatedResult::new(
        users,
        Some(len),
        None,
        None,
        None,
        None,
        None,
    )))
}
