use crate::{
    auth::AuthenticatedUser,
    db::DbPool,
    error::AppError,
    models::{CreateTransactionDto, TransactionType},
    services::PortfolioService,
    templates::{TransactionFormTemplate, TransactionsListTemplate},
};
use askama_axum::IntoResponse;
use axum::{
    extract::{Form, Path, Query, State},
    response::{Redirect, Response},
};
use chrono::{NaiveDate, NaiveDateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct NewTxQuery {
    pub asset_id: Option<String>,
    pub qty: Option<f64>,
    pub price: Option<f64>,
}

pub async fn list_transactions(
    State(db): State<DbPool>,
    user: AuthenticatedUser,
) -> Result<Response, AppError> {
    let transactions =
        PortfolioService::get_user_transactions_with_assets(&db, &user.session.user_id).await?;

    Ok(TransactionsListTemplate {
        user_name: user.session.name,
        transactions,
    }
    .into_response())
}

pub async fn show_create_transaction(
    State(db): State<DbPool>,
    user: AuthenticatedUser,
    Query(query): Query<NewTxQuery>,
) -> Result<Response, AppError> {
    let assets = PortfolioService::get_user_assets(&db, &user.session.user_id).await?;
    let default_date = Utc::now().format("%Y-%m-%d").to_string();

    let prefill_qty = query.qty.map(|q| q.to_string()).unwrap_or_default();
    let prefill_price = query.price.map(|p| format!("{:.2}", p)).unwrap_or_default();

    Ok(TransactionFormTemplate {
        user_name: user.session.name,
        assets,
        selected_asset_id: query.asset_id.unwrap_or_default(),
        prefill_qty,
        prefill_price,
        default_date,
        error_message: None,
    }
    .into_response())
}

pub async fn handle_create_transaction(
    State(db): State<DbPool>,
    user: AuthenticatedUser,
    Form(dto): Form<CreateTransactionDto>,
) -> Result<Response, AppError> {
    let tx_id = Uuid::new_v4().to_string();
    let fees = dto.fees.unwrap_or(0.0);
    let total_amount = (dto.quantity * dto.unit_price) + fees;

    let tx_date: NaiveDateTime = match dto.transaction_date {
        Some(d_str) if !d_str.trim().is_empty() => {
            NaiveDate::parse_from_str(d_str.trim(), "%Y-%m-%d")
                .map(|d| d.and_hms_opt(12, 0, 0).unwrap())
                .unwrap_or_else(|_| Utc::now().naive_utc())
        }
        _ => Utc::now().naive_utc(),
    };

    let tx_type = TransactionType::from_str(&dto.transaction_type);

    match &db {
        DbPool::Postgres(pool) => {
            sqlx::query(
                r#"INSERT INTO transactions (id, user_id, asset_id, transaction_type, quantity, unit_price, total_amount, fees, transaction_date, notes, created_at)
                   VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, CURRENT_TIMESTAMP)"#,
            )
            .bind(&tx_id)
            .bind(&user.session.user_id)
            .bind(&dto.asset_id)
            .bind(tx_type.as_str())
            .bind(dto.quantity)
            .bind(dto.unit_price)
            .bind(total_amount)
            .bind(fees)
            .bind(tx_date)
            .bind(&dto.notes)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;
        }
        DbPool::Sqlite(pool) => {
            sqlx::query(
                r#"INSERT INTO transactions (id, user_id, asset_id, transaction_type, quantity, unit_price, total_amount, fees, transaction_date, notes, created_at)
                   VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP)"#,
            )
            .bind(&tx_id)
            .bind(&user.session.user_id)
            .bind(&dto.asset_id)
            .bind(tx_type.as_str())
            .bind(dto.quantity)
            .bind(dto.unit_price)
            .bind(total_amount)
            .bind(fees)
            .bind(tx_date)
            .bind(&dto.notes)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;
        }
    }

    Ok(Redirect::to("/dashboard").into_response())
}

pub async fn handle_delete_transaction(
    State(db): State<DbPool>,
    user: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<Response, AppError> {
    match &db {
        DbPool::Postgres(pool) => {
            sqlx::query("DELETE FROM transactions WHERE id = $1 AND user_id = $2")
                .bind(&id)
                .bind(&user.session.user_id)
                .execute(pool)
                .await
                .map_err(AppError::Database)?;
        }
        DbPool::Sqlite(pool) => {
            sqlx::query("DELETE FROM transactions WHERE id = ? AND user_id = ?")
                .bind(&id)
                .bind(&user.session.user_id)
                .execute(pool)
                .await
                .map_err(AppError::Database)?;
        }
    }

    Ok(Redirect::to("/transactions").into_response())
}
