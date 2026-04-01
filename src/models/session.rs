use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

/// TBL_SESSION の行
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Session {
    pub sesid: Uuid,
    pub sesus: Uuid,
    pub sesto: String, // SHA-256 ハッシュ済みトークン
    pub sescr: DateTime<Utc>,
}

/// ミドルウェアがリクエスト拡張に格納する認証ユーザー情報
#[derive(Debug, Clone, Serialize)]
pub struct AuthUser {
    pub useid: Uuid,
    pub usenm: String,
}
