use crate::error::AppError;
use sqlx::{postgres::PgPoolOptions, sqlite::SqlitePoolOptions, PgPool, SqlitePool};
use std::time::Duration;
use tracing::{info, warn};

#[derive(Clone, Debug)]
pub enum DbPool {
    Postgres(PgPool),
    Sqlite(SqlitePool),
}

impl DbPool {
    pub async fn init(database_url: &str) -> Result<Self, AppError> {
        // Tenta conectar ao PostgreSQL primeiro
        if database_url.starts_with("postgres://") || database_url.starts_with("postgresql://") {
            info!("Tentando conectar ao banco PostgreSQL em: {}", database_url);
            match PgPoolOptions::new()
                .max_connections(10)
                .acquire_timeout(Duration::from_secs(3))
                .connect(database_url)
                .await
            {
                Ok(pg_pool) => {
                    info!("✅ Conexão com PostgreSQL estabelecida com sucesso!");
                    let db = DbPool::Postgres(pg_pool);
                    db.run_migrations().await?;
                    return Ok(db);
                }
                Err(err) => {
                    warn!(
                        "⚠️ Não foi possível conectar ao PostgreSQL ({:?}). Utilizando banco local SQLite (carteira.db) para desenvolvimento imediato.",
                        err
                    );
                }
            }
        }

        // Fallback para SQLite em modo local caso o PostgreSQL não esteja rodando
        let sqlite_url = "sqlite://carteira.db?mode=rwc";
        info!("Inicializando banco local SQLite: {}", sqlite_url);
        let sqlite_pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(sqlite_url)
            .await
            .map_err(AppError::Database)?;

        let db = DbPool::Sqlite(sqlite_pool);
        db.run_migrations().await?;
        info!("✅ Banco SQLite inicializado e tabelas migradas com sucesso!");
        Ok(db)
    }

    pub async fn run_migrations(&self) -> Result<(), AppError> {
        match self {
            DbPool::Postgres(pool) => {
                info!("Executando migrações no PostgreSQL...");
                sqlx::query(
                    r#"
                    CREATE TABLE IF NOT EXISTS users (
                        id VARCHAR(36) PRIMARY KEY,
                        name VARCHAR(100) NOT NULL,
                        email VARCHAR(150) UNIQUE NOT NULL,
                        password_hash VARCHAR(255) NOT NULL,
                        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );
                    CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

                    CREATE TABLE IF NOT EXISTS assets (
                        id VARCHAR(36) PRIMARY KEY,
                        user_id VARCHAR(36) NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                        ticker VARCHAR(20) NOT NULL,
                        name VARCHAR(150) NOT NULL,
                        asset_class VARCHAR(50) NOT NULL,
                        currency VARCHAR(10) NOT NULL DEFAULT 'BRL',
                        target_percentage DOUBLE PRECISION NOT NULL DEFAULT 0.0,
                        current_price DOUBLE PRECISION NOT NULL DEFAULT 0.0,
                        notes TEXT,
                        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );
                    CREATE INDEX IF NOT EXISTS idx_assets_user_id ON assets(user_id);
                    CREATE INDEX IF NOT EXISTS idx_assets_ticker ON assets(ticker);

                    CREATE TABLE IF NOT EXISTS transactions (
                        id VARCHAR(36) PRIMARY KEY,
                        user_id VARCHAR(36) NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                        asset_id VARCHAR(36) NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
                        transaction_type VARCHAR(20) NOT NULL,
                        quantity DOUBLE PRECISION NOT NULL,
                        unit_price DOUBLE PRECISION NOT NULL,
                        total_amount DOUBLE PRECISION NOT NULL,
                        fees DOUBLE PRECISION NOT NULL DEFAULT 0.0,
                        transaction_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        notes TEXT,
                        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );
                    CREATE INDEX IF NOT EXISTS idx_transactions_user_id ON transactions(user_id);
                    CREATE INDEX IF NOT EXISTS idx_transactions_asset_id ON transactions(asset_id);

                    CREATE TABLE IF NOT EXISTS class_targets (
                        id VARCHAR(36) PRIMARY KEY,
                        user_id VARCHAR(36) NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                        asset_class VARCHAR(50) NOT NULL,
                        target_percentage DOUBLE PRECISION NOT NULL DEFAULT 0.0,
                        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        CONSTRAINT uq_user_class UNIQUE (user_id, asset_class)
                    );
                    "#,
                )
                .execute(pool)
                .await
                .map_err(AppError::Database)?;
            }
            DbPool::Sqlite(pool) => {
                info!("Executando migrações no SQLite...");
                sqlx::query(
                    r#"
                    CREATE TABLE IF NOT EXISTS users (
                        id TEXT PRIMARY KEY,
                        name TEXT NOT NULL,
                        email TEXT UNIQUE NOT NULL,
                        password_hash TEXT NOT NULL,
                        created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );
                    CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

                    CREATE TABLE IF NOT EXISTS assets (
                        id TEXT PRIMARY KEY,
                        user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                        ticker TEXT NOT NULL,
                        name TEXT NOT NULL,
                        asset_class TEXT NOT NULL,
                        currency TEXT NOT NULL DEFAULT 'BRL',
                        target_percentage REAL NOT NULL DEFAULT 0.0,
                        current_price REAL NOT NULL DEFAULT 0.0,
                        notes TEXT,
                        created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );
                    CREATE INDEX IF NOT EXISTS idx_assets_user_id ON assets(user_id);

                    CREATE TABLE IF NOT EXISTS transactions (
                        id TEXT PRIMARY KEY,
                        user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                        asset_id TEXT NOT NULL REFERENCES assets(id) ON DELETE CASCADE,
                        transaction_type TEXT NOT NULL,
                        quantity REAL NOT NULL,
                        unit_price REAL NOT NULL,
                        total_amount REAL NOT NULL,
                        fees REAL NOT NULL DEFAULT 0.0,
                        transaction_date DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        notes TEXT,
                        created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );
                    CREATE INDEX IF NOT EXISTS idx_transactions_user_id ON transactions(user_id);
                    CREATE INDEX IF NOT EXISTS idx_transactions_asset_id ON transactions(asset_id);

                    CREATE TABLE IF NOT EXISTS class_targets (
                        id TEXT PRIMARY KEY,
                        user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                        asset_class TEXT NOT NULL,
                        target_percentage REAL NOT NULL DEFAULT 0.0,
                        updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        UNIQUE(user_id, asset_class)
                    );
                    "#,
                )
                .execute(pool)
                .await
                .map_err(AppError::Database)?;
            }
        }
        info!(" Migrações executadas com sucesso!");
        Ok(())
    }
}
