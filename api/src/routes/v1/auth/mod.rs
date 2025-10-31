use actix_web::Scope;

mod signup;

pub fn auth_scope() -> Scope {
    Scope::new("/auth").service(signup::signup)
}
