use actix_web::Scope;

mod profile;
mod signin;
mod signout;
mod signup;

pub fn auth_scope() -> Scope {
    Scope::new("/auth")
        .service(signup::signup)
        .service(signin::signin)
        .service(signout::signout)
        .service(profile::profile)
}
