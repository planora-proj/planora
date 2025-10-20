use actix_web::{App, HttpServer};

mod config;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::from_env();

    println!("Starting server at http://{}", config.addr());

    HttpServer::new(move || App::new().service(routes::health::health_check))
        .bind(config.addr())?
        .run()
        .await?;

    Ok(())
}
