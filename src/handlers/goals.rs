use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{
        goal::{CreateGoalRequest, Goal, UpdateGoalRequest},
        session::AuthUser,
    },
    AppState,
};

pub async fn list(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
) -> Result<impl IntoResponse, AppError> {
    let goals = sqlx::query_as!(
        Goal,
        r#"SELECT golid, golus, golnm, goldt, golst, golno, golcr, golup
           FROM "TBL_GOAL"
           WHERE golus = $1
           ORDER BY golcr DESC"#,
        auth.useid
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(goals))
}

pub async fn create(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Json(body): Json<CreateGoalRequest>,
) -> Result<impl IntoResponse, AppError> {
    if body.golnm.trim().is_empty() {
        return Err(AppError::BadRequest("目標資格名を入力してください".into()));
    }

    let goal = sqlx::query_as!(
        Goal,
        r#"INSERT INTO "TBL_GOAL" (golus, golnm, goldt, golno)
           VALUES ($1, $2, $3, $4)
           RETURNING golid, golus, golnm, goldt, golst, golno, golcr, golup"#,
        auth.useid,
        body.golnm.trim(),
        body.goldt,
        body.golno.as_deref().filter(|s| !s.trim().is_empty())
    )
    .fetch_one(&state.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(goal)))
}

pub async fn update(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateGoalRequest>,
) -> Result<impl IntoResponse, AppError> {
    // golst のバリデーション
    if let Some(ref st) = body.golst {
        if !["active", "achieved", "abandoned"].contains(&st.as_str()) {
            return Err(AppError::BadRequest("golst は active/achieved/abandoned のいずれかです".into()));
        }
    }

    let goal = sqlx::query_as!(
        Goal,
        r#"UPDATE "TBL_GOAL"
           SET golnm = COALESCE($1, golnm),
               goldt = COALESCE($2, goldt),
               golst = COALESCE($3, golst),
               golno = COALESCE($4, golno),
               golup = NOW()
           WHERE golid = $5 AND golus = $6
           RETURNING golid, golus, golnm, goldt, golst, golno, golcr, golup"#,
        body.golnm.as_deref(),
        body.goldt,
        body.golst.as_deref(),
        body.golno.as_deref(),
        id,
        auth.useid
    )
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(goal))
}

pub async fn delete(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let result = sqlx::query!(
        r#"DELETE FROM "TBL_GOAL" WHERE golid = $1 AND golus = $2"#,
        id, auth.useid
    )
    .execute(&state.pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
