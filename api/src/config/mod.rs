use std::env;

const DEFAULT_SERVER_HOST: &'static str = "127.0.0.1";
const DEFAULT_SERVER_PORT: &'static str = "8080";

pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::from_filename(".env.local").ok();

        let host = env::var("SERVER_HOST").unwrap_or_else(|_| DEFAULT_SERVER_HOST.to_string());
        let port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| DEFAULT_SERVER_PORT.to_string())
            .parse::<u16>()
            .expect("SERVER_PORT must be a valid number");

        Self { host, port }
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
