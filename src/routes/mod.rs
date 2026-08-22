pub mod api_routes;
pub mod asset_routes;
pub mod auth_routes;
pub mod dashboard_routes;
pub mod rebalance_routes;
pub mod transaction_routes;

use crate::db::DbPool;
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;

pub fn create_router(db: DbPool) -> Router {
    Router::new()
        // Rotas Públicas & Autenticação
        .route("/", get(dashboard_routes::root_handler))
        .route("/login", get(auth_routes::show_login))
        .route("/auth/login", post(auth_routes::handle_login))
        .route("/register", get(auth_routes::show_register))
        .route("/auth/register", post(auth_routes::handle_register))
        .route("/auth/logout", post(auth_routes::handle_logout))
        
        // Rotas Principais Autenticadas
        .route("/dashboard", get(dashboard_routes::show_dashboard))
        .route("/rebalance", get(rebalance_routes::show_rebalance_page))

        // Gestão de Ativos
        .route("/assets", get(asset_routes::list_assets))
        .route("/assets/new", get(asset_routes::show_create_asset))
        .route("/assets/new", post(asset_routes::handle_create_asset))
        .route("/assets/:id/edit", get(asset_routes::show_edit_asset))
        .route("/assets/:id/edit", post(asset_routes::handle_edit_asset))
        .route("/assets/:id/delete", post(asset_routes::handle_delete_asset))

        // Transações e Proventos
        .route("/transactions", get(transaction_routes::list_transactions))
        .route("/transactions/new", get(transaction_routes::show_create_transaction))
        .route("/transactions/new", post(transaction_routes::handle_create_transaction))
        .route("/transactions/:id/delete", post(transaction_routes::handle_delete_transaction))

        // REST APIs
        .route("/api/health", get(api_routes::health_check))
        .route("/api/portfolio", get(api_routes::api_get_portfolio))
        .route("/api/rebalance", post(api_routes::api_post_rebalance))
        .route("/api/quotes/sync", post(api_routes::api_sync_quotes))

        // Arquivos Estáticos (CSS, JS, Imagens)
        .nest_service("/static", ServeDir::new("static"))
        
        // Injeção de Estado Global
        .with_state(db)
}
