use axum::{response::Html, routing::get, Router};
mod api;
mod common;
mod ws;

/// Techmino server written in Rust.
#[derive(argh::FromArgs, serde::Deserialize)]
struct Args {
    /// the address to bind to (default: 127.0.0.1:3000)
    #[argh(
        option,
        arg_name = "address",
        short = 'b',
        default = "String::from(\"127.0.0.1:3000\")"
    )]
    bind: String,
    /// path to the configuration file
    #[argh(option, short = 'c', arg_name = "path")]
    config: Option<String>,
    /// log level (default: info)
    #[argh(option, default = "String::from(\"info\")")]
    log_level: String,
}

impl Args {
    fn load() -> Self {
        let args: Self = argh::from_env();
        if let Some(path) = &args.config {
            return serde_json::from_reader(std::fs::File::open(path).unwrap()).unwrap();
        } else {
            return args;
        }
    }
}

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

    let args = Args::load();
    let listener = tokio::net::TcpListener::bind(&args.bind).await.unwrap();
    println!("log level is set to {}", &args.log_level);
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
