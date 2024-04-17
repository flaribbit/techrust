use axum::{
    extract,
    http::HeaderMap,
    response::Json,
    routing::{get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct GetAvatar {
    playerId: i32,
}

#[derive(Serialize)]
struct GetAvatarResponse {
    code: i32,
    data: String,
}

async fn api_avater(extract::Json(payload): extract::Json<GetAvatar>) -> Json<GetAvatarResponse> {
    Json(GetAvatarResponse {
        code: 200,
        data: "".to_string(),
    })
}

async fn api_get_info() {}

async fn api_put_info() {}

async fn api_get_data() {}

async fn api_put_data() {}

pub fn router() -> Router {
    Router::new()
        .route("/avatar", get(api_avater))
        .route("/info", get(api_get_info).put(api_put_info))
        .route("/data", get(api_get_data).put(api_put_data))
}
