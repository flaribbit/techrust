use axum::{
    extract::Query,
    http::HeaderMap,
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct PlayerId {
    playerId: i32,
}

#[derive(Serialize)]
struct Avater {
    code: i32,
    data: String,
}

async fn api_avater(query: Option<Query<PlayerId>>) -> Json<Avater> {
    Json(Avater {
        code: 200,
        data: "".to_string(),
    })
}

/// Response
/// ```json
/// {
///   "code": 200,
///   "data": {
///     "id": 1,
///     "username": "user",
///     "motto": "Techmino is fun!",
///     "region": 0,
///     "avatar_hash": "BASE64_STRING",
///     "avatar_frame": 0,
///     "clan": "",
///     "permission": "Normal",
///     // Below is not included if accessing other player's info
///     "email": "user@example.com",
///     "phone": "123-456-7890"
///   }
/// }
/// ```
#[derive(Serialize)]
struct PlayerInfo {
    id: i32,
    username: String,
    motto: String,
    region: i32,
    avatar_hash: String,
    avatar_frame: i32,
    clan: String,
    permission: String,
    email: Option<String>,
    phone: Option<String>,
}

async fn api_get_info(headers: HeaderMap, query: Option<Query<PlayerId>>) -> Json<PlayerInfo> {
    Json(PlayerInfo {
        id: 1,
        username: "user".to_string(),
        motto: "Techmino is fun!".to_string(),
        region: 0,
        avatar_hash: "".to_string(),
        avatar_frame: 0,
        clan: "".to_string(),
        permission: "Normal".to_string(),
        email: None,
        phone: None,
    })
}

/// ```json
/// {
///   "username": "Tester",
///   "motto": "Techrater is awesome",
///   "region": 1,
///   "avatar": "BASE64_STRING",
///   "avatar_frame": 12,
///   "clan": 26
/// }
/// ```
#[derive(Deserialize)]
struct PutInfo {
    username: String,
    motto: String,
    region: i32,
    avatar: String,
    avatar_frame: i32,
    clan: i32,
}

async fn api_put_info(headers: HeaderMap, Json(info): Json<PutInfo>) -> Json<Avater> {
    Json(Avater {
        code: 200,
        data: "".to_string(),
    })
}

/// put data
/// ```json
/// {
///   "statistics": "ANY_STRING",
///   "ranks": "ANY_STRING",
///   "settings": "ANY_STRING",
///   "keymaps": "ANY_STRING",
///   "touch_1": "ANY_STRING",
///   "touch_2": "ANY_STRING",
///   "touch_3": "ANY_STRING",
///   "extra_1": "ANY_STRING",
///   "extra_2": "ANY_STRING",
///   "extra_3": "ANY_STRING"
/// }
/// ```
#[derive(Deserialize)]
struct PutData {
    statistics: String,
    ranks: String,
    settings: String,
    keymaps: String,
    touch_1: String,
    touch_2: String,
    touch_3: String,
    extra_1: String,
    extra_2: String,
    extra_3: String,
}

async fn api_put_data(headers: HeaderMap, Json(data): Json<PutData>) -> Json<Avater> {
    Json(Avater {
        code: 200,
        data: "".to_string(),
    })
}

/// player data
/// ```json
/// {
///   "code": 200,
///   "data": {
///     "id": 1,
///     "statistics": "ANY_STRING",
///     "ranks": "ANY_STRING",
///     "extra_1": "ANY_STRING",
///     "extra_2": "ANY_STRING",
///     "extra_3": "ANY_STRING",
///     // Below is not included if accessing other player's info
///     "settings": "ANY_STRING",
///     "keymaps": "ANY_STRING",
///     "touch_1": "ANY_STRING",
///     "touch_2": "ANY_STRING",
///     "touch_3": "ANY_STRING"
///   }
/// }
/// ```
#[derive(Serialize)]
struct PlayerData {
    id: i32,
    statistics: String,
    ranks: String,
    extra_1: String,
    extra_2: String,
    extra_3: String,
    settings: Option<String>,
    keymaps: Option<String>,
    touch_1: Option<String>,
    touch_2: Option<String>,
    touch_3: Option<String>,
}

async fn api_get_data(headers: HeaderMap, query: Option<Query<PlayerId>>) -> Json<PlayerData> {
    Json(PlayerData {
        id: 1,
        statistics: "".to_string(),
        ranks: "".to_string(),
        extra_1: "".to_string(),
        extra_2: "".to_string(),
        extra_3: "".to_string(),
        settings: None,
        keymaps: None,
        touch_1: None,
        touch_2: None,
        touch_3: None,
    })
}

pub fn router() -> Router {
    Router::new()
        .route("/avatar", get(api_avater))
        .route("/info", get(api_get_info).put(api_put_info))
        .route("/data", get(api_get_data).put(api_put_data))
}
