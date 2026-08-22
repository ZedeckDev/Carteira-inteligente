use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionType {
    Compra,
    Venda,
    Dividendo,
    Jcp,
    Rendimento,
}

#[allow(dead_code)]
impl TransactionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TransactionType::Compra => "COMPRA",
            TransactionType::Venda => "VENDA",
            TransactionType::Dividendo => "DIVIDENDO",
            TransactionType::Jcp => "JCP",
            TransactionType::Rendimento => "RENDIMENTO",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().trim() {
            "COMPRA" | "BUY" => TransactionType::Compra,
            "VENDA" | "SELL" => TransactionType::Venda,
            "DIVIDENDO" | "DIVIDEND" => TransactionType::Dividendo,
            "JCP" | "JUROS SOBRE CAPITAL PROPRIO" => TransactionType::Jcp,
            "RENDIMENTO" | "RENDIMENTOS" | "FII_DIV" => TransactionType::Rendimento,
            _ => TransactionType::Compra,
        }
    }

    pub fn is_income(&self) -> bool {
        matches!(
            self,
            TransactionType::Dividendo | TransactionType::Jcp | TransactionType::Rendimento
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: String,
    pub user_id: String,
    pub asset_id: String,
    pub transaction_type: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub total_amount: f64,
    pub fees: f64,
    pub transaction_date: NaiveDateTime,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionWithAsset {
    pub id: String,
    pub user_id: String,
    pub asset_id: String,
    pub ticker: String,
    pub asset_name: String,
    pub asset_class: String,
    pub transaction_type: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub total_amount: f64,
    pub fees: f64,
    pub transaction_date: NaiveDateTime,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateTransactionDto {
    pub asset_id: String,
    pub transaction_type: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub fees: Option<f64>,
    pub transaction_date: Option<String>, // yyyy-mm-dd ou vazio
    pub notes: Option<String>,
}
