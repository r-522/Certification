use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// TBL_FAVORITE の行
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Favorite {
    pub favid: Uuid,
    pub favus: Uuid,
    pub favtg: Uuid,
    pub favcr: DateTime<Utc>,
}
