use crate::auth::session::get_session;
use crate::models::UserSession;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header, request::Parts, StatusCode},
    response::{IntoResponse, Redirect, Response},
};
use tower_cookies::Cookies;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub session: UserSession,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookies = Cookies::from_request_parts(parts, state)
            .await
            .map_err(|e| e.into_response())?;

        if let Some(session) = get_session(&cookies) {
            Ok(AuthenticatedUser { session })
        } else {
            // Se a requisição aceita HTML ou é uma navegação direta no browser, redireciona para /login
            let is_browser_request = parts
                .headers
                .get(header::ACCEPT)
                .and_then(|v| v.to_str().ok())
                .map(|v| v.contains("text/html"))
                .unwrap_or(true);

            if is_browser_request {
                Err(Redirect::to("/login").into_response())
            } else {
                Err((
                    StatusCode::UNAUTHORIZED,
                    "Acesso não autorizado. Faça login para continuar.",
                )
                    .into_response())
            }
        }
    }
}

// Extractor opcional para páginas que podem ser acessadas por anônimos ou autenticados
pub struct MaybeUser(pub Option<UserSession>);

#[async_trait]
impl<S> FromRequestParts<S> for MaybeUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookies = Cookies::from_request_parts(parts, state)
            .await
            .map_err(|e| e.into_response())?;

        Ok(MaybeUser(get_session(&cookies)))
    }
}
