use actix_web::Scope;

mod profile;
mod signin;
mod signup;

pub fn auth_scope() -> Scope {
    Scope::new("/auth")
        .service(signup::signup)
        .service(signin::signin)
        .service(profile::profile)
}
