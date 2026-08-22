use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct RegisterDto {
    #[validate(length(min = 3, message = "O nome deve ter no mínimo 3 caracteres"))]
    pub name: String,

    #[validate(email(message = "Informe um e-mail válido"))]
    pub email: String,

    #[validate(length(min = 6, message = "A senha deve ter no mínimo 6 caracteres"))]
    pub password: String,
    pub confirm_password: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct LoginDto {
    #[validate(email(message = "Informe um e-mail válido"))]
    pub email: String,

    #[validate(length(min = 1, message = "Informe a senha"))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub user_id: String,
    pub name: String,
    pub email: String,
}
