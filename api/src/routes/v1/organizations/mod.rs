use actix_web::Scope;

mod create;
mod delete;
mod list;

pub fn organizations_scope() -> Scope {
    Scope::new("/organizations")
        .service(create::create_organization)
        .service(delete::delete_organization)
        .service(list::list_organizations)
}
