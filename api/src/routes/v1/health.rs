use actix_web::{Responder, get};

use arx_gatehouse::common::{ApiResult, time};

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

    ApiResult::to_ok_response("ok", info)
}

#[cfg(test)]
mod tests {
    use actix_web::{App, http::header::ContentType, test};

    use super::*;

    #[actix_web::test]
    async fn test_health_get() {
        let app = test::init_service(App::new().service(health_check)).await;

        let req = test::TestRequest::get()
            .uri("/health")
            .append_header(ContentType::html())
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(
            resp.status().is_success(),
            "Health endpoint did not return success"
        );

        let content_type = resp
            .headers()
            .get("Content-Type")
            .expect("Missing Content-Type header")
            .to_str()
            .unwrap();
        assert_eq!(content_type, ContentType::json().to_string());

        let body: ApiResult<HealthInfo> = test::read_body_json(resp).await;
        assert!(body.success, "Expected success=true");
        assert_eq!(body.message, "ok");
        assert!(
            body.payload.is_some(),
            "Expected payload with timestamp and version"
        );

        let info = body.payload.unwrap();
        assert!(!info.timestamp.is_empty(), "Timestamp should not be empty");
        assert_eq!(info.timestamp, time::current_utc_timestamp());
        assert!(!info.version.is_empty(), "Version should not be empty");
    }
}
