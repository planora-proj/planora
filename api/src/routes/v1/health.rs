use actix_web::{HttpResponse, Responder, get};
use serde::Serialize;

#[derive(Serialize)]
struct Health {
    status: String,
}

#[get("/health")]
async fn health_check() -> impl Responder {
    let response = Health {
        status: "ok".into(),
    };

    HttpResponse::Ok().json(response)
}
