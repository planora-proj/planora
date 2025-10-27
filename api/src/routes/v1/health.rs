use actix_web::{HttpResponse, Responder, get};
use chrono::Utc;

use crate::routes::common::ApiResult;

#[derive(serde::Serialize)]
struct HealthInfo {
    timestamp: String,
    version: &'static str,
}

#[get("/health")]
async fn health_check() -> impl Responder {
    let info = HealthInfo {
        timestamp: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        version: env!("CARGO_PKG_VERSION"),
    };

    HttpResponse::Ok().json(ApiResult::success(info, Some("ok".to_string())))
}
