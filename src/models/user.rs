use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// TBL_USER の行
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub useid: Uuid,
    pub usenm: String,
    pub useml: String, // HMAC-SHA256 ハッシュ済み
    #[serde(skip_serializing)]
    pub usepw: String, // Argon2id ハッシュ済み（レスポンスには含めない）
    pub usecr: DateTime<Utc>,
    pub useup: DateTime<Utc>,
}

/// レスポンス用（パスワードハッシュを除外）
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub useid: Uuid,
    pub usenm: String,
    pub usecr: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        Self {
            useid: u.useid,
            usenm: u.usenm,
            usecr: u.usecr,
        }
    }
}
