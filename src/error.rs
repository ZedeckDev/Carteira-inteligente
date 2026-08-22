use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum AppError {
    #[error("Erro no banco de dados: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Não autorizado: {0}")]
    Unauthorized(String),

    #[error("Requisição inválida: {0}")]
    BadRequest(String),

    #[error("Não encontrado: {0}")]
    NotFound(String),

    #[error("Erro interno no servidor: {0}")]
    Internal(String),

    #[error("Erro de validação: {0}")]
    Validation(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(err) => {
                tracing::error!("Database error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Erro ao processar operação no banco de dados.".to_string(),
                )
            }
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg)
            }
            AppError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
        };

        // Resposta estilizada caso seja requisição de página ou JSON caso seja API
        let html_error = format!(
            r#"<!DOCTYPE html>
            <html lang="pt-BR">
            <head>
                <meta charset="UTF-8">
                <title>Ops! Ocorreu um erro</title>
                <link rel="stylesheet" href="/static/css/style.css">
                <link rel="stylesheet" href="/static/css/components.css">
            </head>
            <body class="error-page">
                <div class="error-container glass-card">
                    <div class="error-icon">⚠️</div>
                    <h2>Atenção</h2>
                    <p class="error-msg">{}</p>
                    <div class="error-actions">
                        <a href="javascript:history.back()" class="btn btn-secondary">← Voltar</a>
                        <a href="/dashboard" class="btn btn-primary">Ir para o Dashboard</a>
                    </div>
                </div>
            </body>
            </html>"#,
            error_message
        );

        (status, Html(html_error)).into_response()
    }
}
