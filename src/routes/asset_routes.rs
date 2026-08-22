use crate::{
    auth::AuthenticatedUser,
    db::DbPool,
    error::AppError,
    models::{Asset, CreateAssetDto, UpdateAssetDto},
    services::{PortfolioService, QuotesService},
    templates::{AssetFormTemplate, AssetsListTemplate},
};
use askama_axum::IntoResponse;
use axum::{
    extract::{Form, Path, State},
    response::{Redirect, Response},
};
use chrono::Utc;
use uuid::Uuid;
use validator::Validate;

pub async fn list_assets(
    State(db): State<DbPool>,
    user: AuthenticatedUser,
) -> Result<Response, AppError> {
    let assets = PortfolioService::get_user_assets(&db, &user.session.user_id).await?;

    Ok(AssetsListTemplate {
        user_name: user.session.name,
        assets,
    }
    .into_response())
}

pub async fn show_create_asset(user: AuthenticatedUser) -> impl IntoResponse {
    let empty_asset = Asset {
        id: String::new(),
        user_id: user.session.user_id.clone(),
        ticker: String::new(),
        name: String::new(),
        asset_class: "Ações".to_string(),
        currency: "BRL".to_string(),
        target_percentage: 0.0,
        current_price: 0.0,
        notes: None,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    AssetFormTemplate {
        user_name: user.session.name,
        is_edit: false,
        asset: empty_asset,
        error_message: None,
    }
}

pub async fn handle_create_asset(
    State(db): State<DbPool>,
    user: AuthenticatedUser,
    Form(dto): Form<CreateAssetDto>,
) -> Result<Response, AppError> {
    if let Err(validation_err) = dto.validate() {
        let err_msg = validation_err
            .field_errors()
            .values()
            .next()
            .and_then(|e| e.first())
            .and_then(|e| e.message.as_ref())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "Dados inválidos.".to_string());

        let empty_asset = Asset {
            id: String::new(),
            user_id: user.session.user_id.clone(),
            ticker: dto.ticker,
            name: dto.name,
            asset_class: dto.asset_class,
            currency: "BRL".to_string(),
            target_percentage: dto.target_percentage.unwrap_or(0.0),
            current_price: dto.current_price.unwrap_or(0.0),
            notes: dto.notes,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        return Ok(AssetFormTemplate {
            user_name: user.session.name,
            is_edit: false,
            asset: empty_asset,
            error_message: Some(err_msg),
        }
        .into_response());
    }

    let asset_id = Uuid::new_v4().to_string();
    let ticker = dto.ticker.trim().to_uppercase();
    let name = dto.name.trim().to_string();
    let asset_class = dto.asset_class.trim().to_string();
    let target_pct = dto.target_percentage.unwrap_or(0.0);

    // Se o preço não for informado, tenta buscar cotação de mercado
    let current_price = match dto.current_price {
        Some(p) if p > 0.0 => p,
        _ => QuotesService::fetch_latest_price(&ticker).await.unwrap_or(0.0),
    };

    match &db {
        DbPool::Postgres(pool) => {
            sqlx::query(
                r#"INSERT INTO assets (id, user_id, ticker, name, asset_class, currency, target_percentage, current_price, notes, created_at, updated_at)
                   VALUES ($1, $2, $3, $4, $5, 'BRL', $6, $7, $8, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"#,
            )
            .bind(&asset_id)
            .bind(&user.session.user_id)
            .bind(&ticker)
            .bind(&name)
            .bind(&asset_class)
            .bind(target_pct)
            .bind(current_price)
            .bind(&dto.notes)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;
        }
        DbPool::Sqlite(pool) => {
            sqlx::query(
                r#"INSERT INTO assets (id, user_id, ticker, name, asset_class, currency, target_percentage, current_price, notes, created_at, updated_at)
                   VALUES (?, ?, ?, ?, ?, 'BRL', ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"#,
            )
            .bind(&asset_id)
            .bind(&user.session.user_id)
            .bind(&ticker)
            .bind(&name)
            .bind(&asset_class)
            .bind(target_pct)
            .bind(current_price)
            .bind(&dto.notes)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;
        }
    }

    Ok(Redirect::to("/assets").into_response())
}

pub async fn show_edit_asset(
    State(db): State<DbPool>,
    user: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<Response, AppError> {
    let asset_opt = match &db {
        DbPool::Postgres(pool) => {
            sqlx::query_as::<_, Asset>(
                "SELECT id, user_id, ticker, name, asset_class, currency, target_percentage, current_price, notes, created_at, updated_at FROM assets WHERE id = $1 AND user_id = $2",
            )
            .bind(&id)
            .bind(&user.session.user_id)
            .fetch_optional(pool)
            .await
            .map_err(AppError::Database)?
        }
        DbPool::Sqlite(pool) => {
            sqlx::query_as::<_, Asset>(
                "SELECT id, user_id, ticker, name, asset_class, currency, target_percentage, current_price, notes, created_at, updated_at FROM assets WHERE id = ? AND user_id = ?",
            )
            .bind(&id)
            .bind(&user.session.user_id)
            .fetch_optional(pool)
            .await
            .map_err(AppError::Database)?
        }
    };

    let asset = match asset_opt {
        Some(a) => a,
        None => return Err(AppError::NotFound("Ativo não encontrado.".to_string())),
    };

    Ok(AssetFormTemplate {
        user_name: user.session.name,
        is_edit: true,
        asset,
        error_message: None,
    }
    .into_response())
}

pub async fn handle_edit_asset(
    State(db): State<DbPool>,
    user: AuthenticatedUser,
    Path(id): Path<String>,
    Form(dto): Form<UpdateAssetDto>,
) -> Result<Response, AppError> {
    let target_pct = dto.target_percentage.unwrap_or(0.0);
    let current_price = dto.current_price.unwrap_or(0.0);
    let ticker = dto.ticker.trim().to_uppercase();

    match &db {
        DbPool::Postgres(pool) => {
            sqlx::query(
                r#"UPDATE assets SET ticker = $1, name = $2, asset_class = $3, target_percentage = $4, current_price = $5, notes = $6, updated_at = CURRENT_TIMESTAMP
                   WHERE id = $7 AND user_id = $8"#,
            )
            .bind(&ticker)
            .bind(dto.name.trim())
            .bind(dto.asset_class.trim())
            .bind(target_pct)
            .bind(current_price)
            .bind(&dto.notes)
            .bind(&id)
            .bind(&user.session.user_id)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;
        }
        DbPool::Sqlite(pool) => {
            sqlx::query(
                r#"UPDATE assets SET ticker = ?, name = ?, asset_class = ?, target_percentage = ?, current_price = ?, notes = ?, updated_at = CURRENT_TIMESTAMP
                   WHERE id = ? AND user_id = ?"#,
            )
            .bind(&ticker)
            .bind(dto.name.trim())
            .bind(dto.asset_class.trim())
            .bind(target_pct)
            .bind(current_price)
            .bind(&dto.notes)
            .bind(&id)
            .bind(&user.session.user_id)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;
        }
    }

    Ok(Redirect::to("/assets").into_response())
}

pub async fn handle_delete_asset(
    State(db): State<DbPool>,
    user: AuthenticatedUser,
    Path(id): Path<String>,
) -> Result<Response, AppError> {
    match &db {
        DbPool::Postgres(pool) => {
            sqlx::query("DELETE FROM assets WHERE id = $1 AND user_id = $2")
                .bind(&id)
                .bind(&user.session.user_id)
                .execute(pool)
                .await
                .map_err(AppError::Database)?;
        }
        DbPool::Sqlite(pool) => {
            sqlx::query("DELETE FROM assets WHERE id = ? AND user_id = ?")
                .bind(&id)
                .bind(&user.session.user_id)
                .execute(pool)
                .await
                .map_err(AppError::Database)?;
        }
    }

    Ok(Redirect::to("/assets").into_response())
}
