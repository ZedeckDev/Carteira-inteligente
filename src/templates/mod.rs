use crate::models::{Asset, PortfolioSummary, RebalanceResult, TransactionWithAsset, UserSession};
use askama::Template;

pub mod filters {
    pub fn fmt_currency(v: &f64) -> ::askama::Result<String> {
        let is_negative = *v < 0.0;
        let abs_v = v.abs();
        let s = format!("{:.2}", abs_v);
        let parts: Vec<&str> = s.split('.').collect();
        let integer_part = parts[0];
        let decimal_part = parts.get(1).unwrap_or(&"00");

        let mut formatted_int = String::new();
        let len = integer_part.len();
        for (i, ch) in integer_part.chars().enumerate() {
            formatted_int.push(ch);
            if (len - i - 1) % 3 == 0 && (len - i - 1) > 0 {
                formatted_int.push('.');
            }
        }

        if is_negative {
            Ok(format!("-{formatted_int},{decimal_part}"))
        } else {
            Ok(format!("{formatted_int},{decimal_part}"))
        }
    }

    pub fn fmt_decimal(v: &f64) -> ::askama::Result<String> {
        Ok(format!("{:.2}", v).replace('.', ","))
    }

    pub fn fmt_date(d: &chrono::NaiveDateTime) -> ::askama::Result<String> {
        Ok(d.format("%d/%m/%Y").to_string())
    }

    pub fn slugify(s: &str) -> ::askama::Result<String> {
        Ok(s.to_lowercase()
            .replace(' ', "-")
            .replace('ç', "c")
            .replace('ã', "a")
            .replace('õ', "o")
            .replace('á', "a")
            .replace('é', "e")
            .replace('í', "i")
            .replace('ó', "o")
            .replace('ú', "u"))
    }
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub user: Option<UserSession>,
}

#[derive(Template)]
#[template(path = "auth/login.html")]
pub struct LoginTemplate {
    pub error_message: Option<String>,
}

#[derive(Template)]
#[template(path = "auth/register.html")]
pub struct RegisterTemplate {
    pub error_message: Option<String>,
}

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub user_name: String,
    pub portfolio: PortfolioSummary,
}

#[derive(Template)]
#[template(path = "assets/list.html")]
pub struct AssetsListTemplate {
    pub user_name: String,
    pub assets: Vec<Asset>,
}

#[derive(Template)]
#[template(path = "assets/form.html")]
pub struct AssetFormTemplate {
    pub user_name: String,
    pub is_edit: bool,
    pub asset: Asset,
    pub error_message: Option<String>,
}

#[derive(Template)]
#[template(path = "transactions/list.html")]
pub struct TransactionsListTemplate {
    pub user_name: String,
    pub transactions: Vec<TransactionWithAsset>,
}

#[derive(Template)]
#[template(path = "transactions/form.html")]
pub struct TransactionFormTemplate {
    pub user_name: String,
    pub assets: Vec<Asset>,
    pub selected_asset_id: String,
    pub prefill_qty: String,
    pub prefill_price: String,
    pub default_date: String,
    pub error_message: Option<String>,
}

#[derive(Template)]
#[template(path = "rebalance.html")]
pub struct RebalanceTemplate {
    pub user_name: String,
    pub amount_input: String,
    pub rebalance_result: Option<RebalanceResult>,
}
