use std::env;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub session_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();

        // 0.0.0.0 permite que plataformas de hospedagem encaminhem tráfego ao container.
        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgrespassword@localhost:5432/carteira_investimentos".to_string());
        let session_secret = env::var("SESSION_SECRET")
            .unwrap_or_else(|_| "super_secret_carteira_investimentos_security_key_32_bytes_long_123456789".to_string());

        Self {
            host,
            port,
            database_url,
            session_secret,
        }
    }
}
