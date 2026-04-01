use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    error::AppError,
    models::session::AuthUser,
    utils::hash::{generate_token, hash_email, hash_password, hash_token, verify_password},
    AppState,
};

#[derive(Deserialize)]
pub struct SignupRequest {
    pub usenm: String,
    pub useml: String,
    pub usepw: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub useml: String,
    pub usepw: String,
}

pub async fn signup(
    State(state): State<AppState>,
    Json(body): Json<SignupRequest>,
) -> Result<impl IntoResponse, AppError> {
    if body.usenm.trim().is_empty() || body.useml.trim().is_empty() || body.usepw.trim().is_empty() {
        return Err(AppError::BadRequest("全ての項目を入力してください".into()));
    }
    if body.usepw.len() < 8 {
        return Err(AppError::BadRequest("パスワードは8文字以上で入力してください".into()));
    }

    let email_hash = hash_email(&body.useml, &state.config.email_hmac_secret);
    let pw_hash = hash_password(&body.usepw)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    // メールアドレス重複チェック
    let existing = sqlx::query!(
        r#"SELECT useid FROM "TBL_USER" WHERE useml = $1"#,
        email_hash
    )
    .fetch_optional(&state.pool)
    .await?;

    if existing.is_some() {
        return Err(AppError::EmailAlreadyExists);
    }

    let user = sqlx::query!(
        r#"INSERT INTO "TBL_USER" (usenm, useml, usepw)
           VALUES ($1, $2, $3)
           RETURNING useid, usenm, usecr"#,
        body.usenm.trim(),
        email_hash,
        pw_hash
    )
    .fetch_one(&state.pool)
    .await?;

    // セッション生成
    let token = generate_token();
    let token_hash = hash_token(&token);

    sqlx::query!(
        r#"INSERT INTO "TBL_SESSION" (sesus, sesto) VALUES ($1, $2)"#,
        user.useid,
        token_hash
    )
    .execute(&state.pool)
    .await?;

    let cookie = format!(
        "session={}; HttpOnly; SameSite=Lax; Path=/; Max-Age=315360000",
        token
    );

    Ok((
        StatusCode::CREATED,
        [(header::SET_COOKIE, cookie)],
        Json(json!({
            "useid": user.useid,
            "usenm": user.usenm,
            "usecr": user.usecr,
        })),
    ))
}

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    if body.useml.trim().is_empty() || body.usepw.trim().is_empty() {
        return Err(AppError::BadRequest("メールアドレスとパスワードを入力してください".into()));
    }

    let email_hash = hash_email(&body.useml, &state.config.email_hmac_secret);

    let user = sqlx::query!(
        r#"SELECT useid, usenm, usepw FROM "TBL_USER" WHERE useml = $1"#,
        email_hash
    )
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::InvalidCredentials)?;

    if !verify_password(&body.usepw, &user.usepw) {
        return Err(AppError::InvalidCredentials);
    }

    let token = generate_token();
    let token_hash = hash_token(&token);

    sqlx::query!(
        r#"INSERT INTO "TBL_SESSION" (sesus, sesto) VALUES ($1, $2)"#,
        user.useid,
        token_hash
    )
    .execute(&state.pool)
    .await?;

    let cookie = format!(
        "session={}; HttpOnly; SameSite=Lax; Path=/; Max-Age=315360000",
        token
    );

    Ok((
        StatusCode::OK,
        [(header::SET_COOKIE, cookie)],
        Json(json!({
            "useid": user.useid,
            "usenm": user.usenm,
        })),
    ))
}

pub async fn logout(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    headers: axum::http::HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    // Cookieヘッダーからセッショントークンを取り出す
    if let Some(token) = headers
        .get("cookie")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| {
            s.split(';').find_map(|p| p.trim().strip_prefix("session=").map(|t| t.to_string()))
        })
    {
        let token_hash = hash_token(&token);
        sqlx::query!(
            r#"DELETE FROM "TBL_SESSION" WHERE sesto = $1 AND sesus = $2"#,
            token_hash,
            auth_user.useid
        )
        .execute(&state.pool)
        .await?;
    }

    let clear_cookie = "session=; HttpOnly; SameSite=Lax; Path=/; Max-Age=0";
    Ok((
        StatusCode::OK,
        [(header::SET_COOKIE, clear_cookie)],
        Json(json!({ "ok": true })),
    ))
}

pub async fn me(
    Extension(auth_user): Extension<AuthUser>,
) -> impl IntoResponse {
    Json(json!({
        "useid": auth_user.useid,
        "usenm": auth_user.usenm,
    }))
}
