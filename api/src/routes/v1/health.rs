use actix_web::{HttpResponse, Responder, get};

use crate::routes::common::{ApiResult, time};

#[cfg_attr(test, derive(serde::Deserialize))]
#[derive(serde::Serialize)]
struct HealthInfo {
    timestamp: String,
    version: String,
}

#[get("/health")]
async fn health_check() -> impl Responder {
    let info = HealthInfo {
        timestamp: time::current_utc_timestamp(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    HttpResponse::Ok().json(ApiResult::success(info, Some("ok".to_string())))
}
