/*
    SPDX-License-Identifier: AGPL-3.0-or-later
    Copyright (C) 2025 Planora
*/

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};

use crate::services::JwtService;

mod config;
mod db;
mod routes;
pub mod services;
mod telemetry;
mod ws;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // config
    let config = config::Config::from_env();
    let is_production_env = config.is_production_env();
    let web_url = config.next_base_url.clone();

    // telemetry
    telemetry::init();

    // database
    let manager = db::manager::DbManager::new();
    manager
        .init_pool("planora", &config.pg_url.clone(), 5)
        .await
        .expect("Failed to connect to Postgres");

    // services
    let jwt_service = JwtService::from_env();

    // actix server
    tracing::info!("Starting server at http://{}", config.addr());
    HttpServer::new(move || {
        use actix_web::http::header;

        // For development: allow all cross-origin requests (CORS)
        // In production, restrict CORS to specific allowed origins for security.
        let cors = if !is_production_env {
            Cors::permissive()
        } else {
            Cors::default()
                .allowed_origin(&web_url)
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                .allowed_headers(vec![
                    header::AUTHORIZATION,
                    header::ACCEPT,
                    header::CONTENT_TYPE,
                ])
                .max_age(3600)
        };

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(manager.clone()))
            .app_data(web::Data::new(jwt_service.clone()))
            .route("/ws", web::get().to(ws::ws))
            .service(routes::v1::v1_scope())
    })
    .bind(config.addr())?
    .run()
    .await?;

    Ok(())
}
