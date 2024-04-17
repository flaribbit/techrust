use axum::{
    response::{Html, Json},
    routing::get,
    Router,
};
use serde::Serialize;
mod api;

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
    let app = Router::new()
        .route("/", get(handler))
        .route("/json", get(handler2))
        .nest("/techmino/api/v1", api_v1());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn main() {
    tokio::runtime::Builder::new_current_thread()
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
