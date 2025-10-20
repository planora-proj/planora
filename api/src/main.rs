use actix_web::{App, HttpServer};

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || App::new().service(routes::health::health_check))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await?;

    Ok(())
}
