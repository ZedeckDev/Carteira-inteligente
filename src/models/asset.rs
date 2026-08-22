use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AssetClass {
    Acoes,
    Fiis,
    RendaFixa,
    Etfs,
    Cripto,
    Internacional,
    Outros,
}

#[allow(dead_code)]
impl AssetClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            AssetClass::Acoes => "Ações",
            AssetClass::Fiis => "FIIs",
            AssetClass::RendaFixa => "Renda Fixa",
            AssetClass::Etfs => "ETFs",
            AssetClass::Cripto => "Cripto",
            AssetClass::Internacional => "Internacional",
            AssetClass::Outros => "Outros",
        }
    }

    pub fn from_str_name(s: &str) -> Self {
        match s.to_lowercase().trim() {
            "ações" | "acoes" | "acao" | "ações brasil" => AssetClass::Acoes,
            "fiis" | "fii" | "fundos imobiliarios" | "fundos imobiliários" => AssetClass::Fiis,
            "renda fixa" | "rendafixa" | "cdb" | "tesouro" => AssetClass::RendaFixa,
            "etfs" | "etf" => AssetClass::Etfs,
            "cripto" | "criptomoedas" | "crypto" | "btc" => AssetClass::Cripto,
            "internacional" | "stocks" | "reits" => AssetClass::Internacional,
            _ => AssetClass::Outros,
        }
    }

    pub fn all() -> Vec<&'static str> {
        vec!["Ações", "FIIs", "Renda Fixa", "ETFs", "Cripto", "Internacional", "Outros"]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Asset {
    pub id: String,
    pub user_id: String,
    pub ticker: String,
    pub name: String,
    pub asset_class: String,
    pub currency: String,
    pub target_percentage: f64,
    pub current_price: f64,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateAssetDto {
    #[validate(length(min = 1, message = "O código/ticker é obrigatório"))]
    pub ticker: String,

    #[validate(length(min = 1, message = "O nome do ativo é obrigatório"))]
    pub name: String,

    pub asset_class: String,
    pub target_percentage: Option<f64>,
    pub current_price: Option<f64>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateAssetDto {
    pub ticker: String,
    pub name: String,
    pub asset_class: String,
    pub target_percentage: Option<f64>,
    pub current_price: Option<f64>,
    pub notes: Option<String>,
}
