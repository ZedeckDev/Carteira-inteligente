mod auth;
mod config;
mod db;
mod error;
mod models;
mod routes;
mod services;
mod templates;

use config::Config;
use db::DbPool;
use routes::create_router;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inicializa Logs com Tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "carteira_inteligente=debug,tower_http=debug,axum=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("🚀 Iniciando CARTEIRA - Gestão de Investimentos...");

    // Carrega configurações
    let config = Config::from_env();

    // Inicializa Banco de Dados e Migrações
    let db = DbPool::init(&config.database_url).await?;

    // Configura o Router com middlewares (Cookies, CORS, Trace)
    let app = create_router(db)
        .layer(CookieManagerLayer::new())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    // Inicia Servidor HTTP Axum
    let host_ip = config.host.parse().unwrap_or([127, 0, 0, 1].into());
    let addr = SocketAddr::new(host_ip, config.port);
    info!("🌟 Servidor pronto e rodando em: http://{}", addr);
    info!("👉 Acesse http://localhost:{} no seu navegador para utilizar a aplicação!", config.port);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
