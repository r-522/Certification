use std::sync::Arc;

use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use vercel_runtime::{run, Body, Error, Request, Response};

use shikaku::{
    config::Config,
    handlers::{auth, catalog, certifications, favorites, goals, users},
    middleware::auth::require_auth,
    AppState,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let config = Config::from_env()
        .map_err(|e| Error::from(format!("環境変数の読み込みに失敗しました: {}", e)))?;

    let pool = sqlx::PgPool::connect(&config.database_url)
        .await
        .map_err(|e| Error::from(format!("DB接続に失敗しました: {}", e)))?;

    let state = AppState {
        pool,
        config: Arc::new(config),
    };

    let app = build_router(state);

    // vercel_runtime::Request → http::Request<axum::body::Body> へ変換
    let (parts, body) = req.into_parts();
    let body_bytes: bytes::Bytes = match body {
        Body::Empty => bytes::Bytes::new(),
        Body::Text(s) => bytes::Bytes::from(s.into_bytes()),
        Body::Binary(b) => bytes::Bytes::from(b),
    };
    let axum_req =
        http::Request::from_parts(parts, axum::body::Body::from(body_bytes));

    // axum でルーティング処理
    use tower::ServiceExt;
    let axum_resp = app
        .into_service()
        .oneshot(axum_req)
        .await
        .map_err(|e| Error::from(e.to_string()))?;

    // axum response → vercel_runtime response へ変換
    let (resp_parts, resp_body) = axum_resp.into_parts();
    let resp_bytes = axum::body::to_bytes(resp_body, usize::MAX)
        .await
        .map_err(|e| Error::from(e.to_string()))?;

    Ok(http::Response::from_parts(
        resp_parts,
        Body::Binary(resp_bytes.to_vec()),
    ))
}

fn build_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 認証不要ルート
    let public = Router::new()
        .route("/api/auth/signup", post(auth::signup))
        .route("/api/auth/login", post(auth::login));

    // 認証必須ルート
    let protected = Router::new()
        .route("/api/auth/logout", post(auth::logout))
        .route("/api/auth/me", get(auth::me))
        .route("/api/certifications", get(certifications::list).post(certifications::create))
        .route("/api/certifications/:id", put(certifications::update).delete(certifications::delete))
        .route("/api/catalog", get(catalog::search))
        .route("/api/goals", get(goals::list).post(goals::create))
        .route("/api/goals/:id", put(goals::update).delete(goals::delete))
        .route("/api/users", get(users::list))
        .route("/api/favorites/:user_id", post(favorites::toggle))
        .layer(middleware::from_fn_with_state(state.clone(), require_auth));

    Router::new()
        .merge(public)
        .merge(protected)
        .layer(cors)
        .with_state(state)
}
