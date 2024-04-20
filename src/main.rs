use axum::{
    response::{Html, Json},
    routing::get,
    Router,
};
use serde::Serialize;
mod api;
mod common;
mod ws;

#[derive(Serialize)]
struct Message {
    message: String,
}

fn api_v1() -> Router {
    Router::new()
        .merge(api::notice::router())
        .merge(api::auth::router())
        .merge(api::player::router())
}

async fn async_main() {
    use std::sync::Arc;
    let (app_state, rx) = common::AppState::new();
    let app_state = Arc::new(app_state);
    let state2 = app_state.clone();
    let state3 = app_state.clone();
    let app = Router::new()
        .route("/", get(handler))
        .route("/json", get(handler2))
        .route("/techmino/ws/v1", get(ws::handler))
        .with_state(app_state)
        .nest("/techmino/api/v1", api_v1());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    tokio::spawn(ws::response::sender_task(state2, rx));
    tokio::spawn(async move {
        loop {
            let data = common::MessageData {
                player_id: 0,
                message: "hello".to_string(),
            };
            state3.tx.send(data).unwrap();
            println!("send message");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });
    axum::serve(listener, app).await.unwrap();
}

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main());
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn handler2() -> Json<Message> {
    Json(Message {
        message: "Hello, World!".to_string(),
    })
}
