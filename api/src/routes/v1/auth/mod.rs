use actix_web::Scope;

pub fn auth_scope() -> Scope {
    Scope::new("/auth")
}
