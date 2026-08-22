use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionSummary {
    pub asset_id: String,
    pub ticker: String,
    pub name: String,
    pub asset_class: String,
    pub quantity: f64,
    pub average_price: f64,
    pub current_price: f64,
    pub total_invested: f64,
    pub current_total_value: f64,
    pub profit_loss: f64,
    pub profit_loss_percentage: f64,
    pub total_income: f64, // Proventos recebidos
    pub current_weight_percentage: f64,
    pub target_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassSummary {
    pub asset_class: String,
    pub total_invested: f64,
    pub current_total_value: f64,
    pub current_weight_percentage: f64,
    pub target_percentage: f64,
    pub profit_loss: f64,
    pub profit_loss_percentage: f64,
    pub total_income: f64,
    pub asset_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioSummary {
    pub total_invested: f64,
    pub current_total_value: f64,
    pub total_profit_loss: f64,
    pub profit_loss_percentage: f64,
    pub total_dividends_received: f64,
    pub monthly_dividends_current_month: f64,
    pub yield_on_cost: f64,
    pub positions: Vec<PositionSummary>,
    pub classes: Vec<ClassSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ClassTarget {
    pub id: String,
    pub user_id: String,
    pub asset_class: String,
    pub target_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebalanceRecommendation {
    pub asset_id: String,
    pub ticker: String,
    pub name: String,
    pub asset_class: String,
    pub current_price: f64,
    pub current_quantity: f64,
    pub current_value: f64,
    pub current_percentage: f64,
    pub target_percentage: f64,
    pub distance_to_target_percentage: f64, // target - current
    pub recommended_buy_amount: f64,
    pub recommended_quantity: f64,
    pub expected_new_value: f64,
    pub expected_new_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebalanceResult {
    pub contribution_amount: f64,
    pub total_portfolio_before: f64,
    pub total_portfolio_after: f64,
    pub recommendations: Vec<RebalanceRecommendation>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RebalanceInputDto {
    pub amount: f64,
}
