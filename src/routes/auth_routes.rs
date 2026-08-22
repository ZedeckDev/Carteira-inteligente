use crate::{
    auth::{clear_session_cookie, hash_password, set_session_cookie, verify_password},
    db::DbPool,
    error::AppError,
    models::{LoginDto, RegisterDto, User},
    templates::{LoginTemplate, RegisterTemplate},
};
use askama_axum::IntoResponse;
use axum::{
    extract::{Form, State},
    response::{Redirect, Response},
};
use tower_cookies::Cookies;
use uuid::Uuid;
use validator::Validate;

pub async fn show_login() -> impl IntoResponse {
    LoginTemplate {
        error_message: None,
    }
}

pub async fn handle_login(
    State(db): State<DbPool>,
    cookies: Cookies,
    Form(dto): Form<LoginDto>,
) -> Result<Response, AppError> {
    if let Err(validation_err) = dto.validate() {
        let err_msg = validation_err
            .field_errors()
            .values()
            .next()
            .and_then(|e| e.first())
            .and_then(|e| e.message.as_ref())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "Dados de login inválidos.".to_string());

        return Ok(LoginTemplate {
            error_message: Some(err_msg),
        }
        .into_response());
    }

    let user_opt = match &db {
        DbPool::Postgres(pool) => {
            sqlx::query_as::<_, User>("SELECT id, name, email, password_hash, created_at FROM users WHERE email = $1")
                .bind(&dto.email.to_lowercase())
                .fetch_optional(pool)
                .await
                .map_err(AppError::Database)?
        }
        DbPool::Sqlite(pool) => {
            sqlx::query_as::<_, User>("SELECT id, name, email, password_hash, created_at FROM users WHERE email = ?")
                .bind(&dto.email.to_lowercase())
                .fetch_optional(pool)
                .await
                .map_err(AppError::Database)?
        }
    };

    let user = match user_opt {
        Some(u) => u,
        None => {
            return Ok(LoginTemplate {
                error_message: Some("E-mail ou senha incorretos.".to_string()),
            }
            .into_response());
        }
    };

    if !verify_password(&dto.password, &user.password_hash)? {
        return Ok(LoginTemplate {
            error_message: Some("E-mail ou senha incorretos.".to_string()),
        }
        .into_response());
    }

    // Login efetuado com sucesso -> define cookie de sessão seguro
    set_session_cookie(&cookies, &user.id, &user.name, &user.email);
    Ok(Redirect::to("/dashboard").into_response())
}

pub async fn show_register() -> impl IntoResponse {
    RegisterTemplate {
        error_message: None,
    }
}

pub async fn handle_register(
    State(db): State<DbPool>,
    cookies: Cookies,
    Form(dto): Form<RegisterDto>,
) -> Result<Response, AppError> {
    if let Err(validation_err) = dto.validate() {
        let err_msg = validation_err
            .field_errors()
            .values()
            .next()
            .and_then(|e| e.first())
            .and_then(|e| e.message.as_ref())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "Dados de cadastro inválidos.".to_string());

        return Ok(RegisterTemplate {
            error_message: Some(err_msg),
        }
        .into_response());
    }

    if dto.password != dto.confirm_password {
        return Ok(RegisterTemplate {
            error_message: Some("As senhas não coincidem.".to_string()),
        }
        .into_response());
    }

    let email = dto.email.trim().to_lowercase();

    // Verifica se e-mail já existe
    let exists = match &db {
        DbPool::Postgres(pool) => {
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE email = $1")
                .bind(&email)
                .fetch_one(pool)
                .await
                .map_err(AppError::Database)?
                > 0
        }
        DbPool::Sqlite(pool) => {
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE email = ?")
                .bind(&email)
                .fetch_one(pool)
                .await
                .map_err(AppError::Database)?
                > 0
        }
    };

    if exists {
        return Ok(RegisterTemplate {
            error_message: Some("Este e-mail já está cadastrado na plataforma.".to_string()),
        }
        .into_response());
    }

    let user_id = Uuid::new_v4().to_string();
    let hashed_pw = hash_password(&dto.password)?;

    match &db {
        DbPool::Postgres(pool) => {
            sqlx::query(
                "INSERT INTO users (id, name, email, password_hash, created_at) VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)",
            )
            .bind(&user_id)
            .bind(&dto.name)
            .bind(&email)
            .bind(&hashed_pw)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;
        }
        DbPool::Sqlite(pool) => {
            sqlx::query(
                "INSERT INTO users (id, name, email, password_hash, created_at) VALUES (?, ?, ?, ?, CURRENT_TIMESTAMP)",
            )
            .bind(&user_id)
            .bind(&dto.name)
            .bind(&email)
            .bind(&hashed_pw)
            .execute(pool)
            .await
            .map_err(AppError::Database)?;
        }
    }

    // Configura sessão e redireciona direto para o Dashboard
    set_session_cookie(&cookies, &user_id, &dto.name, &email);
    Ok(Redirect::to("/dashboard").into_response())
}

pub async fn handle_logout(cookies: Cookies) -> impl IntoResponse {
    clear_session_cookie(&cookies);
    Redirect::to("/login")
}
