pub mod response;
use crate::common::{AppState, User};
use axum::extract::{
    ws::{Message, WebSocket, WebSocketUpgrade},
    State,
};
use std::sync::Arc;

pub async fn handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> axum::response::Response {
    ws.on_upgrade(|socket| websocket(socket, state))
}

async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    use futures_util::stream::StreamExt;
    let (sender, mut receiver) = stream.split();
    let sender = Arc::new(tokio::sync::Mutex::new(sender));
    state.online_users.lock().unwrap().insert(
        0,
        User {
            id: 0,
            name: "test".to_string(),
            sender: sender.clone(),
        },
    );
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(text) = message {
            println!("received: {}", text);
            match serde_json::from_str(&text) {
                Ok(json) => handle_message(&json, &state, &sender),
                Err(e) => println!("error: {}", e),
            }
        }
    }
    // disconnected
    println!("disconnected");
}

type Sender = futures_util::stream::SplitSink<WebSocket, Message>;
fn handle_message(
    json: &serde_json::Value,
    state: &Arc<AppState>,
    sender: &Arc<tokio::sync::Mutex<Sender>>,
) {
    let action_id = json["action_id"].as_i64().unwrap_or_default();
    macro_rules! handlers {
        ($($key:expr => $value:ident),* $(,)?) => {
            match action_id {
                $( $key => $value(json, state, sender), )*
                _ => println!("unknown action_id: {}", action_id),
            }
        };
    }
    use response::*;
    handlers! {
        1000 => global_online_count,
        1100 => match_end,
        1101 => match_ready,
        1102 => match_start,
        1200 => player_config,
        1201 => player_finish,
        1202 => player_group,
        1203 => player_ready,
        1204 => player_role,
        1205 => player_state,
        1206 => player_stream,
        1207 => player_type,
        1300 => room_chat,
        1301 => room_create,
        1302 => room_data_get,
        1303 => room_data_update,
        1304 => room_info_get,
        1305 => room_info_update,
        1306 => room_join,
        1307 => room_kick,
        1308 => room_leave,
        1309 => room_list,
        1310 => room_password,
        1311 => room_remove,
    }
}
