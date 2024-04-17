use axum::{
    response::{Html, Json},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Message {
    message: String,
}

async fn async_main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/json", get(handler2));

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
