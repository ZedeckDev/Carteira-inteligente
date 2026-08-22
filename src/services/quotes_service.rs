use crate::{db::DbPool, error::AppError};
use serde::Deserialize;

pub struct QuotesService;

#[derive(Debug, Deserialize)]
struct BrapiResponse {
    results: Option<Vec<BrapiQuote>>,
}

#[derive(Debug, Deserialize)]
struct BrapiQuote {
    #[allow(dead_code)]
    symbol: Option<String>,
    #[serde(rename = "regularMarketPrice")]
    regular_market_price: Option<f64>,
}

impl QuotesService {
    /// Atualiza o preço atual de um ativo consultando fontes de mercado ou simulador
    pub async fn fetch_latest_price(ticker: &str) -> Option<f64> {
        let clean_ticker = ticker.trim().to_uppercase();
        
        // Tenta API pública de cotações Brapi
        let url = format!("https://brapi.dev/api/quote/{}", clean_ticker);
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(3))
            .build()
            .ok()?;

        if let Ok(resp) = client.get(&url).send().await {
            if resp.status().is_success() {
                if let Ok(data) = resp.json::<BrapiResponse>().await {
                    if let Some(results) = data.results {
                        if let Some(first) = results.into_iter().next() {
                            if let Some(price) = first.regular_market_price {
                                if price > 0.0 {
                                    return Some(price);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Cotações de referência para tickers populares conhecidos caso offline
        match clean_ticker.as_str() {
            "PETR4" => Some(38.50),
            "VALE3" => Some(62.80),
            "ITUB4" => Some(34.20),
            "BBAS3" => Some(27.40),
            "WEGE3" => Some(51.30),
            "MXRF11" => Some(10.25),
            "HGLG11" => Some(164.50),
            "XPML11" => Some(112.30),
            "KNCR11" => Some(102.10),
            "IVVB11" => Some(335.00),
            "BTC" | "BTCBRL" | "BITCOIN" => Some(385000.00),
            "ETH" | "ETHBRL" | "ETHEREUM" => Some(18200.00),
            "TESOURO IPCA+" | "TESOURO SELIC" => Some(1000.00),
            _ => None,
        }
    }

    /// Atualiza os preços de todos os ativos cadastrados no banco
    pub async fn update_all_user_asset_prices(db: &DbPool, user_id: &str) -> Result<usize, AppError> {
        let assets = match db {
            DbPool::Postgres(pool) => {
                sqlx::query_as::<_, (String, String)>(
                    "SELECT id, ticker FROM assets WHERE user_id = $1",
                )
                .bind(user_id)
                .fetch_all(pool)
                .await
                .map_err(AppError::Database)?
            }
            DbPool::Sqlite(pool) => {
                sqlx::query_as::<_, (String, String)>(
                    "SELECT id, ticker FROM assets WHERE user_id = ?",
                )
                .bind(user_id)
                .fetch_all(pool)
                .await
                .map_err(AppError::Database)?
            }
        };

        let mut updated_count = 0;
        for (asset_id, ticker) in assets {
            if let Some(price) = Self::fetch_latest_price(&ticker).await {
                match db {
                    DbPool::Postgres(pool) => {
                        let _ = sqlx::query(
                            "UPDATE assets SET current_price = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2",
                        )
                        .bind(price)
                        .bind(&asset_id)
                        .execute(pool)
                        .await;
                    }
                    DbPool::Sqlite(pool) => {
                        let _ = sqlx::query(
                            "UPDATE assets SET current_price = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?",
                        )
                        .bind(price)
                        .bind(&asset_id)
                        .execute(pool)
                        .await;
                    }
                }
                updated_count += 1;
            }
        }

        Ok(updated_count)
    }
}
