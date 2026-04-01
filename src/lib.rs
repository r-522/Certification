use std::sync::Arc;

pub mod config;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod utils;

use config::Config;

/// axum ハンドラー間で共有するアプリケーション状態
#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub config: Arc<Config>,
}
