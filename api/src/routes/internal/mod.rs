use actix_web::{Scope, web};

mod users;

pub fn internal_routes() -> Scope {
    web::scope("/internal").service(users::get_users)
}
