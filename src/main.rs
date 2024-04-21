use axum::{response::Html, routing::get, Router};
mod api;
mod common;
mod ws;

fn api_v1() -> Router {
    Router::new()
        .merge(api::notice::router())
        .merge(api::auth::router())
        .merge(api::player::router())
}

async fn async_main() {
    use std::sync::Arc;
    let app_state = Arc::new(common::AppState::new());
    let app = Router::new()
        .route("/", get(handler))
        .route("/techmino/ws/v1", get(ws::handler))
        .with_state(app_state)
        .nest("/techmino/api/v1", api_v1());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
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
