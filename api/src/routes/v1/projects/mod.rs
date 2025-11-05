use actix_web::Scope;

mod create;
mod delete;
mod helper;
mod list;

pub fn projects_scope() -> Scope {
    Scope::new("/projects")
        .service(create::create_project)
        .service(delete::delete_project)
        .service(list::list_projects)
}
