use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// TBL_CERTIFICATION の行
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Cert {
    pub cerid: Uuid,
    pub cerus: Uuid,
    pub cernm: String,
    pub cerdt: Option<NaiveDate>,
    pub cerno: Option<String>,
    pub cercr: DateTime<Utc>,
    pub cerup: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCertRequest {
    pub cernm: String,
    pub cerdt: Option<NaiveDate>,
    pub cerno: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCertRequest {
    pub cernm: Option<String>,
    pub cerdt: Option<NaiveDate>,
    pub cerno: Option<String>,
}
