use crate::{
    auth::AuthenticatedUser,
    db::DbPool,
    error::AppError,
    services::{PortfolioService, RebalanceService},
    templates::RebalanceTemplate,
};
use askama_axum::IntoResponse;
use axum::{
    extract::{Query, State},
    response::Response,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RebalanceQuery {
    pub amount: Option<f64>,
}

pub async fn show_rebalance_page(
    State(db): State<DbPool>,
    user: AuthenticatedUser,
    Query(query): Query<RebalanceQuery>,
) -> Result<Response, AppError> {
    let portfolio = PortfolioService::calculate_portfolio(&db, &user.session.user_id).await?;

    let (amount_input, rebalance_result) = if let Some(amt) = query.amount {
        if amt > 0.0 {
            let res = RebalanceService::calculate_rebalance(&portfolio.positions, amt);
            (format!("{:.2}", amt), Some(res))
        } else {
            ("2000.00".to_string(), None)
        }
    } else {
        ("2000.00".to_string(), None)
    };

    Ok(RebalanceTemplate {
        user_name: user.session.name,
        amount_input,
        rebalance_result,
    }
    .into_response())
}
