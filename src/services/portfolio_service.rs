use crate::{
    db::DbPool,
    error::AppError,
    models::{
        Asset, AssetClass, ClassSummary, ClassTarget, PositionSummary, PortfolioSummary,
        Transaction, TransactionType, TransactionWithAsset,
    },
};
use std::collections::HashMap;

pub struct PortfolioService;

impl PortfolioService {
    /// Carrega todos os ativos de um usuário
    pub async fn get_user_assets(db: &DbPool, user_id: &str) -> Result<Vec<Asset>, AppError> {
        match db {
            DbPool::Postgres(pool) => {
                let assets = sqlx::query_as::<_, Asset>(
                    "SELECT id, user_id, ticker, name, asset_class, currency, target_percentage, current_price, notes, created_at, updated_at FROM assets WHERE user_id = $1 ORDER BY ticker ASC",
                )
                .bind(user_id)
                .fetch_all(pool)
                .await
                .map_err(AppError::Database)?;
                Ok(assets)
            }
            DbPool::Sqlite(pool) => {
                let assets = sqlx::query_as::<_, Asset>(
                    "SELECT id, user_id, ticker, name, asset_class, currency, target_percentage, current_price, notes, created_at, updated_at FROM assets WHERE user_id = ? ORDER BY ticker ASC",
                )
                .bind(user_id)
                .fetch_all(pool)
                .await
                .map_err(AppError::Database)?;
                Ok(assets)
            }
        }
    }

    /// Carrega todas as transações de um usuário
    pub async fn get_user_transactions(
        db: &DbPool,
        user_id: &str,
    ) -> Result<Vec<Transaction>, AppError> {
        match db {
            DbPool::Postgres(pool) => {
                let txs = sqlx::query_as::<_, Transaction>(
                    "SELECT id, user_id, asset_id, transaction_type, quantity, unit_price, total_amount, fees, transaction_date, notes, created_at FROM transactions WHERE user_id = $1 ORDER BY transaction_date DESC, created_at DESC",
                )
                .bind(user_id)
                .fetch_all(pool)
                .await
                .map_err(AppError::Database)?;
                Ok(txs)
            }
            DbPool::Sqlite(pool) => {
                let txs = sqlx::query_as::<_, Transaction>(
                    "SELECT id, user_id, asset_id, transaction_type, quantity, unit_price, total_amount, fees, transaction_date, notes, created_at FROM transactions WHERE user_id = ? ORDER BY transaction_date DESC, created_at DESC",
                )
                .bind(user_id)
                .fetch_all(pool)
                .await
                .map_err(AppError::Database)?;
                Ok(txs)
            }
        }
    }

    /// Carrega transações enriquecidas com dados do ativo
    pub async fn get_user_transactions_with_assets(
        db: &DbPool,
        user_id: &str,
    ) -> Result<Vec<TransactionWithAsset>, AppError> {
        let assets = Self::get_user_assets(db, user_id).await?;
        let asset_map: HashMap<String, Asset> = assets.into_iter().map(|a| (a.id.clone(), a)).collect();
        let transactions = Self::get_user_transactions(db, user_id).await?;

        let mut enriched = Vec::new();
        for tx in transactions {
            let (ticker, name, class) = match asset_map.get(&tx.asset_id) {
                Some(a) => (a.ticker.clone(), a.name.clone(), a.asset_class.clone()),
                None => ("-".to_string(), "Ativo Removido".to_string(), "Outros".to_string()),
            };

            enriched.push(TransactionWithAsset {
                id: tx.id,
                user_id: tx.user_id,
                asset_id: tx.asset_id,
                ticker,
                asset_name: name,
                asset_class: class,
                transaction_type: tx.transaction_type,
                quantity: tx.quantity,
                unit_price: tx.unit_price,
                total_amount: tx.total_amount,
                fees: tx.fees,
                transaction_date: tx.transaction_date,
                notes: tx.notes,
            });
        }

        Ok(enriched)
    }

    /// Carrega metas de classe de ativos
    pub async fn get_class_targets(
        db: &DbPool,
        user_id: &str,
    ) -> Result<Vec<ClassTarget>, AppError> {
        match db {
            DbPool::Postgres(pool) => {
                let targets = sqlx::query_as::<_, ClassTarget>(
                    "SELECT id, user_id, asset_class, target_percentage FROM class_targets WHERE user_id = $1",
                )
                .bind(user_id)
                .fetch_all(pool)
                .await
                .map_err(AppError::Database)?;
                Ok(targets)
            }
            DbPool::Sqlite(pool) => {
                let targets = sqlx::query_as::<_, ClassTarget>(
                    "SELECT id, user_id, asset_class, target_percentage FROM class_targets WHERE user_id = ?",
                )
                .bind(user_id)
                .fetch_all(pool)
                .await
                .map_err(AppError::Database)?;
                Ok(targets)
            }
        }
    }

    /// Calcula o resumo consolidado da carteira
    pub async fn calculate_portfolio(
        db: &DbPool,
        user_id: &str,
    ) -> Result<PortfolioSummary, AppError> {
        let assets = Self::get_user_assets(db, user_id).await?;
        let transactions = Self::get_user_transactions(db, user_id).await?;
        let class_targets = Self::get_class_targets(db, user_id).await?;

        let class_target_map: HashMap<String, f64> = class_targets
            .into_iter()
            .map(|ct| (ct.asset_class, ct.target_percentage))
            .collect();

        // Agrupa transações por ativo (ordenadas da mais antiga para a mais recente para cálculo de preço médio)
        let mut txs_by_asset: HashMap<String, Vec<Transaction>> = HashMap::new();
        for tx in transactions.into_iter().rev() {
            txs_by_asset.entry(tx.asset_id.clone()).or_default().push(tx);
        }

        let mut positions = Vec::new();
        let mut total_portfolio_current_value = 0.0;
        let mut total_portfolio_invested = 0.0;
        let mut total_dividends_all = 0.0;

        for asset in &assets {
            let asset_txs = txs_by_asset.get(&asset.id).cloned().unwrap_or_default();

            let mut current_qty = 0.0;
            let mut total_cost = 0.0;
            let mut asset_income = 0.0;

            for tx in &asset_txs {
                let tx_type = TransactionType::from_str(&tx.transaction_type);
                match tx_type {
                    TransactionType::Compra => {
                        let cost = (tx.quantity * tx.unit_price) + tx.fees;
                        current_qty += tx.quantity;
                        total_cost += cost;
                    }
                    TransactionType::Venda => {
                        if current_qty > 0.0 {
                            let avg_price = total_cost / current_qty;
                            let sold_qty = tx.quantity.min(current_qty);
                            current_qty -= sold_qty;
                            total_cost -= sold_qty * avg_price;
                            if current_qty <= 0.0 {
                                current_qty = 0.0;
                                total_cost = 0.0;
                            }
                        }
                    }
                    TransactionType::Dividendo
                    | TransactionType::Jcp
                    | TransactionType::Rendimento => {
                        asset_income += tx.total_amount;
                        total_dividends_all += tx.total_amount;
                    }
                }
            }

            let avg_price = if current_qty > 0.0 {
                total_cost / current_qty
            } else {
                0.0
            };

            let current_value = current_qty * asset.current_price;
            let profit_loss = if current_qty > 0.0 {
                current_value - total_cost
            } else {
                0.0
            };
            let profit_loss_pct = if total_cost > 0.0 {
                (profit_loss / total_cost) * 100.0
            } else {
                0.0
            };

            total_portfolio_current_value += current_value;
            total_portfolio_invested += total_cost;

            positions.push(PositionSummary {
                asset_id: asset.id.clone(),
                ticker: asset.ticker.clone(),
                name: asset.name.clone(),
                asset_class: asset.asset_class.clone(),
                quantity: current_qty,
                average_price: avg_price,
                current_price: asset.current_price,
                total_invested: total_cost,
                current_total_value: current_value,
                profit_loss,
                profit_loss_percentage: profit_loss_pct,
                total_income: asset_income,
                current_weight_percentage: 0.0, // calculado no passo seguinte
                target_percentage: asset.target_percentage,
            });
        }

        // Calcula os pesos percentuais de cada posição
        for pos in &mut positions {
            if total_portfolio_current_value > 0.0 {
                pos.current_weight_percentage =
                    (pos.current_total_value / total_portfolio_current_value) * 100.0;
            }
        }

        // Agrupa por classe de ativo
        let mut class_map: HashMap<String, ClassSummary> = HashMap::new();
        for class_name in AssetClass::all() {
            let target_pct = class_target_map.get(class_name).copied().unwrap_or(0.0);
            class_map.insert(
                class_name.to_string(),
                ClassSummary {
                    asset_class: class_name.to_string(),
                    total_invested: 0.0,
                    current_total_value: 0.0,
                    current_weight_percentage: 0.0,
                    target_percentage: target_pct,
                    profit_loss: 0.0,
                    profit_loss_percentage: 0.0,
                    total_income: 0.0,
                    asset_count: 0,
                },
            );
        }

        for pos in &positions {
            let entry = class_map.entry(pos.asset_class.clone()).or_insert_with(|| {
                ClassSummary {
                    asset_class: pos.asset_class.clone(),
                    total_invested: 0.0,
                    current_total_value: 0.0,
                    current_weight_percentage: 0.0,
                    target_percentage: 0.0,
                    profit_loss: 0.0,
                    profit_loss_percentage: 0.0,
                    total_income: 0.0,
                    asset_count: 0,
                }
            });

            entry.total_invested += pos.total_invested;
            entry.current_total_value += pos.current_total_value;
            entry.profit_loss += pos.profit_loss;
            entry.total_income += pos.total_income;
            if pos.quantity > 0.0 {
                entry.asset_count += 1;
            }
        }

        let mut classes: Vec<ClassSummary> = class_map.into_values().collect();
        for cls in &mut classes {
            if total_portfolio_current_value > 0.0 {
                cls.current_weight_percentage =
                    (cls.current_total_value / total_portfolio_current_value) * 100.0;
            }
            if cls.total_invested > 0.0 {
                cls.profit_loss_percentage = (cls.profit_loss / cls.total_invested) * 100.0;
            }
        }

        classes.sort_by(|a, b| b.current_total_value.partial_cmp(&a.current_total_value).unwrap_or(std::cmp::Ordering::Equal));

        let total_profit_loss = total_portfolio_current_value - total_portfolio_invested;
        let profit_loss_percentage = if total_portfolio_invested > 0.0 {
            (total_profit_loss / total_portfolio_invested) * 100.0
        } else {
            0.0
        };

        let yield_on_cost = if total_portfolio_invested > 0.0 {
            (total_dividends_all / total_portfolio_invested) * 100.0
        } else {
            0.0
        };

        Ok(PortfolioSummary {
            total_invested: total_portfolio_invested,
            current_total_value: total_portfolio_current_value,
            total_profit_loss,
            profit_loss_percentage,
            total_dividends_received: total_dividends_all,
            monthly_dividends_current_month: 0.0,
            yield_on_cost,
            positions,
            classes,
        })
    }
}
