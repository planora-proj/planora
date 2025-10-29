use actix_web::{Scope, web};

mod auth;
mod health;

pub fn v1_scope() -> Scope {
    let scope = web::scope("/v1")
        .service(health::health_check)
        .service(auth::auth_scope());
    scope
}
