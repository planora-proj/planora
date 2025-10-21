/*
    SPDX-License-Identifier: AGPL-3.0-or-later
    Copyright (C) 2025 Planora
*/

use actix_web::{App, HttpServer};

mod config;
mod routes;
mod telemetry;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::from_env();

    // telemetry
    telemetry::init();

    tracing::info!("Starting server at http://{}", config.addr());
    HttpServer::new(move || App::new().service(routes::health::health_check))
        .bind(config.addr())?
        .run()
        .await?;

    Ok(())
}
