use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::{error::AppError, models::session::AuthUser, AppState};

/// お気に入りをトグル（登録済みなら削除、未登録なら追加）
pub async fn toggle(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(target_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    if auth.useid == target_id {
        return Err(AppError::BadRequest("自分自身をお気に入りにはできません".into()));
    }

    // 対象ユーザーの存在確認
    let target = sqlx::query!(
        r#"SELECT useid FROM "TBL_USER" WHERE useid = $1"#,
        target_id
    )
    .fetch_optional(&state.pool)
    .await?;

    if target.is_none() {
        return Err(AppError::NotFound);
    }

    // 既存のお気に入り確認
    let existing = sqlx::query!(
        r#"SELECT favid FROM "TBL_FAVORITE" WHERE favus = $1 AND favtg = $2"#,
        auth.useid,
        target_id
    )
    .fetch_optional(&state.pool)
    .await?;

    if existing.is_some() {
        // 削除
        sqlx::query!(
            r#"DELETE FROM "TBL_FAVORITE" WHERE favus = $1 AND favtg = $2"#,
            auth.useid,
            target_id
        )
        .execute(&state.pool)
        .await?;

        Ok(Json(json!({ "is_favorite": false })))
    } else {
        // 追加
        sqlx::query!(
            r#"INSERT INTO "TBL_FAVORITE" (favus, favtg) VALUES ($1, $2)"#,
            auth.useid,
            target_id
        )
        .execute(&state.pool)
        .await?;

        Ok(Json(json!({ "is_favorite": true })))
    }
}
