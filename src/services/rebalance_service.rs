use crate::models::{PositionSummary, RebalanceRecommendation, RebalanceResult};

pub struct RebalanceService;

impl RebalanceService {
    /// Executa o algoritmo de Aporte Inteligente
    pub fn calculate_rebalance(
        positions: &[PositionSummary],
        contribution_amount: f64,
    ) -> RebalanceResult {
        let total_current = positions.iter().map(|p| p.current_total_value).sum::<f64>();
        let target_portfolio_value = total_current + contribution_amount;

        // Se a soma das metas percentuais for 0 ou nenhum ativo tiver meta, divide uniformemente
        let total_target_pct: f64 = positions.iter().map(|p| p.target_percentage).sum();
        let has_valid_targets = total_target_pct > 0.0;

        let mut deficits: Vec<(usize, f64)> = Vec::new();
        let mut total_deficit = 0.0;

        for (idx, pos) in positions.iter().enumerate() {
            let target_pct = if has_valid_targets {
                pos.target_percentage
            } else if !positions.is_empty() {
                100.0 / (positions.len() as f64)
            } else {
                0.0
            };

            let ideal_value = target_portfolio_value * (target_pct / 100.0);
            let deficit = (ideal_value - pos.current_total_value).max(0.0);

            if deficit > 0.0 {
                deficits.push((idx, deficit));
                total_deficit += deficit;
            }
        }

        let mut recommendations = Vec::new();

        for (idx, pos) in positions.iter().enumerate() {
            let target_pct = if has_valid_targets {
                pos.target_percentage
            } else if !positions.is_empty() {
                100.0 / (positions.len() as f64)
            } else {
                0.0
            };

            let current_pct = if total_current > 0.0 {
                (pos.current_total_value / total_current) * 100.0
            } else {
                0.0
            };

            let distance = target_pct - current_pct;

            // Calcula alocação proporcional ao déficit
            let allocated_amount = if total_deficit > 0.0 && contribution_amount > 0.0 {
                let asset_deficit = deficits
                    .iter()
                    .find(|(i, _)| *i == idx)
                    .map(|(_, d)| *d)
                    .unwrap_or(0.0);

                (asset_deficit / total_deficit) * contribution_amount
            } else {
                0.0
            };

            let recommended_qty = if pos.current_price > 0.0 {
                if pos.asset_class == "Cripto" || pos.asset_class == "Renda Fixa" {
                    // Cripto e Renda Fixa permitem frações
                    (allocated_amount / pos.current_price * 10000.0).round() / 10000.0
                } else {
                    // Ações e FIIs: cotas inteiras
                    (allocated_amount / pos.current_price).floor()
                }
            } else {
                0.0
            };

            let actual_allocated = if pos.asset_class == "Cripto" || pos.asset_class == "Renda Fixa" {
                allocated_amount
            } else {
                recommended_qty * pos.current_price
            };

            let expected_new_value = pos.current_total_value + actual_allocated;
            let expected_new_pct = if target_portfolio_value > 0.0 {
                (expected_new_value / target_portfolio_value) * 100.0
            } else {
                0.0
            };

            recommendations.push(RebalanceRecommendation {
                asset_id: pos.asset_id.clone(),
                ticker: pos.ticker.clone(),
                name: pos.name.clone(),
                asset_class: pos.asset_class.clone(),
                current_price: pos.current_price,
                current_quantity: pos.quantity,
                current_value: pos.current_total_value,
                current_percentage: current_pct,
                target_percentage: target_pct,
                distance_to_target_percentage: distance,
                recommended_buy_amount: actual_allocated,
                recommended_quantity: recommended_qty,
                expected_new_value,
                expected_new_percentage: expected_new_pct,
            });
        }

        // Ordena por maior recomendação de compra / maior distância da meta
        recommendations.sort_by(|a, b| {
            b.recommended_buy_amount
                .partial_cmp(&a.recommended_buy_amount)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        RebalanceResult {
            contribution_amount,
            total_portfolio_before: total_current,
            total_portfolio_after: total_current + contribution_amount,
            recommendations,
        }
    }
}
