use axum::{extract::{Query, State}, response::IntoResponse, Extension, Json};
use serde::Deserialize;

use crate::{error::AppError, models::session::AuthUser, AppState};

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
}

pub async fn search(
    State(state): State<AppState>,
    _auth: Extension<AuthUser>,
    Query(params): Query<SearchQuery>,
) -> Result<impl IntoResponse, AppError> {
    let q = params.q.unwrap_or_default();
    let pattern = format!("%{}%", q.trim());

    let items = sqlx::query_as!(
        crate::models::catalog::Catalog,
        r#"SELECT catid, catnm, catcr
           FROM "TBL_CATALOG"
           WHERE catnm ILIKE $1
           ORDER BY catnm
           LIMIT 20"#,
        pattern
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(items))
}
