use crate::{
    auth::{AuthenticatedUser, MaybeUser},
    db::DbPool,
    error::AppError,
    services::PortfolioService,
    templates::{DashboardTemplate, IndexTemplate},
};
use askama_axum::IntoResponse;
use axum::{
    extract::State,
    response::Response,
};

pub async fn root_handler(MaybeUser(user): MaybeUser) -> impl IntoResponse {
    IndexTemplate { user }
}

pub async fn show_dashboard(
    State(db): State<DbPool>,
    user: AuthenticatedUser,
) -> Result<Response, AppError> {
    let portfolio = PortfolioService::calculate_portfolio(&db, &user.session.user_id).await?;

    Ok(DashboardTemplate {
        user_name: user.session.name,
        portfolio,
    }
    .into_response())
}
