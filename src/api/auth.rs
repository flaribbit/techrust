use axum::{
    http::HeaderMap,
    routing::{get, post, put},
    Json, Router,
};
use serde::Serialize;

#[derive(Serialize)]
struct CommonResponse {
    code: i32,
}

async fn api_check(headers: HeaderMap) -> Json<CommonResponse> {
    match headers.get("x-access-token") {
        Some(token) => {
            // TODO: check the token
            return Json(CommonResponse { code: 200 });
        }
        None => return Json(CommonResponse { code: 401 }),
    }
}

#[derive(Serialize)]
struct TokenData {
    refreshToken: String,
    accessToken: String,
}

#[derive(Serialize)]
struct RefreshResponse {
    code: i32,
    data: TokenData,
}

async fn api_refresh(headers: HeaderMap) {}

async fn api_verify_email() {}

async fn api_seed_email() {}

async fn api_login_email() {}

async fn api_reset_email() {}

async fn api_migrate_email() {}

async fn api_deactivate_email() {}

pub fn router() -> Router {
    Router::new()
        .route("/check", get(api_check))
        .route("/refresh", get(api_refresh))
        .route("/verify/email", post(api_verify_email))
        .route("/seed/email", post(api_seed_email))
        .route("/login/email", post(api_login_email))
        .route("/reset/email", put(api_reset_email))
        .route("/migrate/email", put(api_migrate_email))
        .route("/deactivate/email", post(api_deactivate_email))
}
