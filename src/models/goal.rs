use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum GoalStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "achieved")]
    Achieved,
    #[serde(rename = "abandoned")]
    Abandoned,
}

impl std::fmt::Display for GoalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GoalStatus::Active => write!(f, "active"),
            GoalStatus::Achieved => write!(f, "achieved"),
            GoalStatus::Abandoned => write!(f, "abandoned"),
        }
    }
}

/// TBL_GOAL の行
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Goal {
    pub golid: Uuid,
    pub golus: Uuid,
    pub golnm: String,
    pub goldt: Option<NaiveDate>,
    pub golst: String,
    pub golno: Option<String>,
    pub golcr: DateTime<Utc>,
    pub golup: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateGoalRequest {
    pub golnm: String,
    pub goldt: Option<NaiveDate>,
    pub golno: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateGoalRequest {
    pub golnm: Option<String>,
    pub goldt: Option<NaiveDate>,
    pub golst: Option<String>,
    pub golno: Option<String>,
}
