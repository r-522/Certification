use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// TBL_CATALOG の行（資格マスタ）
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Catalog {
    pub catid: Uuid,
    pub catnm: String,
    pub catcr: DateTime<Utc>,
}
