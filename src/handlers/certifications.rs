use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{cert::{CreateCertRequest, UpdateCertRequest}, session::AuthUser},
    AppState,
};

pub async fn list(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<impl IntoResponse, AppError> {
    let certs = sqlx::query_as!(
        crate::models::cert::Cert,
        r#"SELECT cerid, cerus, cernm, cerdt, cerno, cercr, cerup
           FROM "TBL_CERTIFICATION"
           WHERE cerus = $1
           ORDER BY cercr DESC"#,
        auth.useid
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(certs))
}

pub async fn create(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<CreateCertRequest>,
) -> Result<impl IntoResponse, AppError> {
    if body.cernm.trim().is_empty() {
        return Err(AppError::BadRequest("資格名を入力してください".into()));
    }

    // 資格マスタに未登録なら追加
    sqlx::query!(
        r#"INSERT INTO "TBL_CATALOG" (catnm)
           VALUES ($1)
           ON CONFLICT (catnm) DO NOTHING"#,
        body.cernm.trim()
    )
    .execute(&state.pool)
    .await?;

    let cert = sqlx::query_as!(
        crate::models::cert::Cert,
        r#"INSERT INTO "TBL_CERTIFICATION" (cerus, cernm, cerdt, cerno)
           VALUES ($1, $2, $3, $4)
           RETURNING cerid, cerus, cernm, cerdt, cerno, cercr, cerup"#,
        auth.useid,
        body.cernm.trim(),
        body.cerdt,
        body.cerno.as_deref().filter(|s| !s.trim().is_empty())
    )
    .fetch_one(&state.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(cert)))
}

pub async fn update(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateCertRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 所有者確認
    let existing = sqlx::query!(
        r#"SELECT cerid FROM "TBL_CERTIFICATION" WHERE cerid = $1 AND cerus = $2"#,
        id, auth.useid
    )
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;
    let _ = existing;

    let cert = sqlx::query_as!(
        crate::models::cert::Cert,
        r#"UPDATE "TBL_CERTIFICATION"
           SET cernm = COALESCE($1, cernm),
               cerdt = COALESCE($2, cerdt),
               cerno = COALESCE($3, cerno),
               cerup = NOW()
           WHERE cerid = $4 AND cerus = $5
           RETURNING cerid, cerus, cernm, cerdt, cerno, cercr, cerup"#,
        body.cernm.as_deref(),
        body.cerdt,
        body.cerno.as_deref(),
        id,
        auth.useid
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(cert))
}

pub async fn delete(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let result = sqlx::query!(
        r#"DELETE FROM "TBL_CERTIFICATION" WHERE cerid = $1 AND cerus = $2"#,
        id, auth.useid
    )
    .execute(&state.pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
