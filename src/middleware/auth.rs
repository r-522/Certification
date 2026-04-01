use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

use crate::{models::session::AuthUser, AppState};

/// Cookieからセッショントークンを取り出す
fn extract_session_token(req: &Request) -> Option<String> {
    req.headers()
        .get("cookie")
        .and_then(|v| v.to_str().ok())
        .and_then(|cookie| {
            cookie.split(';').find_map(|part| {
                part.trim().strip_prefix("session=").map(|s| s.to_string())
            })
        })
}

/// 認証ミドルウェア: CookieのセッションTokenを検証し AuthUser を Extension に格納する
pub async fn require_auth(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = extract_session_token(&req).ok_or(StatusCode::UNAUTHORIZED)?;
    let token_hash = crate::utils::hash::hash_token(&token);

    let row = sqlx::query!(
        r#"SELECT s.sesus, u.usenm
           FROM "TBL_SESSION" s
           JOIN "TBL_USER" u ON u.useid = s.sesus
           WHERE s.sesto = $1"#,
        token_hash
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(AuthUser {
        useid: row.sesus,
        usenm: row.usenm,
    });

    Ok(next.run(req).await)
}
