use axum::{extract::State, response::IntoResponse, Extension, Json};
use serde::Serialize;
use uuid::Uuid;

use crate::{error::AppError, models::session::AuthUser, AppState};

#[derive(Debug, Serialize)]
pub struct UserSummary {
    pub useid: Uuid,
    pub usenm: String,
    pub cert_count: i64,
    pub achieved_count: i64,
    pub has_achievement: bool,
    pub is_favorite: bool,
}

pub async fn list(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<impl IntoResponse, AppError> {
    // ユーザー一覧 + 資格数 + 達成目標数 + お気に入り状態を一括取得
    let rows = sqlx::query!(
        r#"SELECT
               u.useid,
               u.usenm,
               COUNT(DISTINCT c.cerid)  AS cert_count,
               COUNT(DISTINCT g.golid)  AS achieved_count,
               EXISTS (
                   SELECT 1 FROM "TBL_FAVORITE" f
                   WHERE f.favus = $1 AND f.favtg = u.useid
               ) AS is_favorite
           FROM "TBL_USER" u
           LEFT JOIN "TBL_CERTIFICATION" c ON c.cerus = u.useid
           LEFT JOIN "TBL_GOAL" g ON g.golus = u.useid AND g.golst = 'achieved'
           GROUP BY u.useid, u.usenm
           ORDER BY u.usenm"#,
        auth.useid
    )
    .fetch_all(&state.pool)
    .await?;

    let users: Vec<UserSummary> = rows
        .into_iter()
        .map(|r| {
            let cert_count = r.cert_count.unwrap_or(0);
            let achieved_count = r.achieved_count.unwrap_or(0);
            UserSummary {
                useid: r.useid,
                usenm: r.usenm,
                cert_count,
                achieved_count,
                has_achievement: achieved_count > 0,
                is_favorite: r.is_favorite.unwrap_or(false),
            }
        })
        .collect();

    // お気に入りを先頭に、次いでusenm昇順
    let mut sorted = users;
    sorted.sort_by(|a, b| b.is_favorite.cmp(&a.is_favorite).then(a.usenm.cmp(&b.usenm)));

    Ok(Json(sorted))
}
