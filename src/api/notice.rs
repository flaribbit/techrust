#![allow(non_snake_case)]
use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct GetNotice {
    lastCount: Option<i32>,
    pageIndex: Option<i32>,
    pageSize: Option<i32>,
    language: String,
}

#[derive(Deserialize)]
struct DeleteNotice {
    noticeId: i32,
}

#[derive(Deserialize)]
struct PostNotice {
    en_us: String,
    zh_cn: String,
}

#[derive(Serialize)]
struct PostNoticeResponse {
    noticeId: i32,
}

#[derive(Serialize)]
struct DeleteNoticeResponse {
    code: i32,
}

#[derive(Serialize)]
struct GetNoticeResponse {
    code: i32,
    data: Vec<Notice>,
}

#[derive(Serialize)]
struct Notice {
    id: i32,
    timestamp: String,
    content: String,
}

async fn api_get(Json(_payload): Json<GetNotice>) -> Json<GetNoticeResponse> {
    // TODO: process the payload
    Json(GetNoticeResponse {
        code: 200,
        data: vec![Notice {
            id: 1,
            timestamp: "2021-01-01 00:00:00".to_string(),
            content: "Hello, World!".to_string(),
        }],
    })
}

async fn api_post(Json(_payload): Json<PostNotice>) -> Json<PostNoticeResponse> {
    // TODO: process the payload
    Json(PostNoticeResponse { noticeId: 0 })
}

async fn api_delete(Json(_payload): Json<DeleteNotice>) -> Json<DeleteNoticeResponse> {
    // TODO: process the payload
    Json(DeleteNoticeResponse { code: 200 })
}

async fn api_put(Json(_payload): Json<PostNotice>) -> Json<DeleteNoticeResponse> {
    Json(DeleteNoticeResponse { code: 200 })
}

pub fn router() -> Router {
    Router::new().route(
        "/notice",
        get(api_get).post(api_post).delete(api_delete).put(api_put),
    )
}
