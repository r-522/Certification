use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("認証が必要です")]
    Unauthorized,

    #[error("アクセスが禁止されています")]
    Forbidden,

    #[error("リソースが見つかりません")]
    NotFound,

    #[error("リクエストが不正です: {0}")]
    BadRequest(String),

    #[error("メールアドレスまたはパスワードが違います")]
    InvalidCredentials,

    #[error("このメールアドレスは既に使用されています")]
    EmailAlreadyExists,

    #[error("データベースエラー: {0}")]
    Database(#[from] sqlx::Error),

    #[error("内部エラー: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", self.to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "FORBIDDEN", self.to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "NOT_FOUND", self.to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg.clone()),
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "INVALID_CREDENTIALS", self.to_string()),
            AppError::EmailAlreadyExists => (StatusCode::CONFLICT, "EMAIL_EXISTS", self.to_string()),
            AppError::Database(e) => {
                tracing::error!("DB error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR", "データベースエラーが発生しました".to_string())
            }
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "内部エラーが発生しました".to_string())
            }
        };

        (status, Json(json!({ "error": message, "code": code }))).into_response()
    }
}
