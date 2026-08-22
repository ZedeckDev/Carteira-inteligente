use crate::{
    auth::AuthenticatedUser,
    db::DbPool,
    error::AppError,
    models::{PortfolioSummary, RebalanceInputDto, RebalanceResult},
    services::{PortfolioService, QuotesService, RebalanceService},
};
use axum::{
    extract::{Json, State},
    response::{IntoResponse, Redirect, Response},
};
use serde_json::json;

pub async fn api_get_portfolio(
    State(db): State<DbPool>,
    user: AuthenticatedUser,
) -> Result<Json<PortfolioSummary>, AppError> {
    let portfolio = PortfolioService::calculate_portfolio(&db, &user.session.user_id).await?;
    Ok(Json(portfolio))
}

pub async fn api_post_rebalance(
    State(db): State<DbPool>,
    user: AuthenticatedUser,
    Json(dto): Json<RebalanceInputDto>,
) -> Result<Json<RebalanceResult>, AppError> {
    let portfolio = PortfolioService::calculate_portfolio(&db, &user.session.user_id).await?;
    let result = RebalanceService::calculate_rebalance(&portfolio.positions, dto.amount);
    Ok(Json(result))
}

pub async fn api_sync_quotes(
    State(db): State<DbPool>,
    user: AuthenticatedUser,
) -> Result<Response, AppError> {
    let count = QuotesService::update_all_user_asset_prices(&db, &user.session.user_id).await?;
    tracing::info!("Atualizadas {} cotações para o usuário {}", count, user.session.user_id);
    Ok(Redirect::to("/dashboard").into_response())
}

pub async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "Carteira Inteligente API",
        "version": env!("CARGO_PKG_VERSION")
    }))
}
