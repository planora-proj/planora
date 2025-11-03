use actix_web::Scope;

pub fn organizations_scope() -> Scope {
    Scope::new("/organizations")
}
