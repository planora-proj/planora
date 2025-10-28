use std::env;

const DEFAULT_SERVER_HOST: &'static str = "127.0.0.1";
const DEFAULT_SERVER_PORT: &'static str = "8080";
const DEFAULT_NEXT_APP_IP: &'static str = "http://localhost:3000";
const DEFAULT_APP_ENVIRONMENT: &'static str = "development";
const DEFAULT_PG_DATABASE_URL: &'static str = "postgres://planora:planora@localhost:5432/planora";

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub next_base_url: String,
    pub app_env: String,
    pub pg_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::from_filename(".env.local").ok();

        let host = env::var("SERVER_HOST").unwrap_or_else(|_| DEFAULT_SERVER_HOST.to_string());
        let port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| DEFAULT_SERVER_PORT.to_string())
            .parse::<u16>()
            .expect("SERVER_PORT must be a valid number");
        let next_base_url =
            env::var("NEXT_PUBLIC_BASE_URL").unwrap_or(DEFAULT_NEXT_APP_IP.to_string());
        let app_env = env::var("APP_ENV").unwrap_or(DEFAULT_APP_ENVIRONMENT.to_string());
        let pg_url = env::var("PG_DATABASE_URL").unwrap_or(DEFAULT_PG_DATABASE_URL.to_string());

        Self {
            host,
            port,
            next_base_url,
            app_env,
            pg_url,
        }
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn is_production_env(&self) -> bool {
        matches!(self.app_env.as_str(), "production")
    }
}
