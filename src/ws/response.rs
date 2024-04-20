use crate::common::{AppState, MessageData, Sender};
use axum::extract::ws::Message;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn sender_task(state: Arc<AppState>, rx: std::sync::mpsc::Receiver<MessageData>) {
    use futures_util::SinkExt;
    while let Ok(MessageData { player_id, message }) = rx.recv() {
        println!("send message to player: {}", player_id);
        let message = Message::Text(message);
        let user_sender = state
            .online_users
            .lock()
            .unwrap()
            .get(&player_id)
            .map(|user| user.sender.clone());
        // 这个锁搞不好会一直锁着全局状态
        if let Some(sender) = user_sender {
            sender.lock().await.send(message).await.unwrap();
        }
    }
}

fn send_json(state: &AppState, json: serde_json::Value) {
    let message = Message::Text(json.to_string());
    state
        .tx
        .send(MessageData {
            player_id: 0,
            message: json.to_string(),
        })
        .unwrap();
}

pub fn global_online_count(
    _json: &serde_json::Value,
    state: &AppState,
    sender: &Arc<Mutex<Sender>>,
) {
    let count = state.online_users.lock().unwrap().len();
    let data = json!({
        "action": 1000,
        "errno": 0,
        "data": count
    });
    send_json(state, data);
}

pub fn match_end(json: &serde_json::Value, state: &AppState, sender: &Arc<Mutex<Sender>>) {
    let data = json!({
        "action": 1100,
        "errno": 0
    });
}

pub fn match_ready(json: &serde_json::Value, state: &AppState, sender: &Arc<Mutex<Sender>>) {
    let data = json!({
        "action": 1101,
        "errno": 0
    });
}

pub fn match_start(json: &serde_json::Value, state: &AppState, sender: &Arc<Mutex<Sender>>) {
    let seed = json["seed"].as_i64().unwrap_or_default();
    let data = json!({
        "action": 1102,
        "errno": 0,
        "data": {
            "seed": seed
        }
    });
}

pub fn player_config(json: &serde_json::Value, state: &AppState, sender: &Arc<Mutex<Sender>>) {
    let player_id = json["playerId"].as_i64().unwrap_or_default();
    let data = json!({
        "action": 1200,
        "errno": 0,
        "data": {
            "playerId": player_id,
            "config": json["config"]
        }
    });
}

pub fn player_finish(json: &serde_json::Value, state: &AppState, sender: &Arc<Mutex<Sender>>) {
    let data = json!({
        "action": 1201,
        "errno": 0,
        "data": {
            "playerId": 123,
            "data": json["data"]
        }
    });
}

pub fn player_group(json: &serde_json::Value, state: &AppState, sender: &Arc<Mutex<Sender>>) {
    let player_id = json["playerId"].as_i64().unwrap_or_default();
    let group_id = json["group"].as_i64().unwrap_or_default();
    let data = json!({
        "action": 1202,
        "errno": 0,
        "data": {
            "playerId": player_id,
            "group": group_id
        }
    });
}

pub fn player_ready(json: &serde_json::Value, state: &AppState, sender: &Arc<Mutex<Sender>>) {
    let player_id = json["playerId"].as_i64().unwrap_or_default();
    let is_ready = json["isReady"].as_bool().unwrap_or_default();
    let data = json!({
        "action": 1203,
        "errno": 0,
        "data": {
            "playerId": player_id,
            "isReady": is_ready
        }
    });
}

pub fn player_role(json: &serde_json::Value, state: &AppState, sender: &Arc<Mutex<Sender>>) {
    let player_id = json["playerId"].as_i64().unwrap_or_default();
    let role = json["role"].as_str().unwrap_or_default();
    let data = json!({
        "action": 1204,
        "errno": 0,
        "data": {
            "playerId": player_id,
            "role": role
        }
    });
}

pub fn player_state(json: &serde_json::Value, state: &AppState, sender: &Arc<Mutex<Sender>>) {
    let player_id = json["playerId"].as_i64().unwrap_or_default();
    let state = json["customState"].as_str().unwrap_or_default();
    let data = json!({
        "action": 1205,
        "errno": 0,
        "data": {
            "playerId": player_id,
            "customState": state
        }
    });
}

pub fn player_stream(json: &serde_json::Value, state: &AppState, sender: &Arc<Mutex<Sender>>) {
    let player_id = json["playerId"].as_i64().unwrap_or_default();
    let stream = json["stream"].as_str().unwrap_or_default();
    let data = json!({
        "action": 1206,
        "errno": 0,
        "data": {
            "playerId": player_id,
            "stream": stream
        }
    });
}

pub fn player_type(json: &serde_json::Value, state: &AppState, sender: &Arc<Mutex<Sender>>) {
    let player_id = json["playerId"].as_i64().unwrap_or_default();
    let player_type = json["type"].as_str().unwrap_or_default();
    let data = json!({
        "action": 1207,
        "errno": 0,
        "data": {
            "playerId": player_id,
            "type": player_type
        }
    });
}

pub fn room_chat(json: &serde_json::Value, state: &AppState, sender: &Arc<Mutex<Sender>>) {
    let player_id = json["playerId"].as_i64().unwrap_or_default();
    let message = json["message"].as_str().unwrap_or_default();
    let data = json!({
        "action": 1300,
        "errno": 0,
        "data": {
            "playerId": player_id,
            "message": message
        }
    });
}

pub fn room_create(json: &serde_json::Value, state: &AppState, sender: &Arc<Mutex<Sender>>) {
    let data = json!({
        "action": 1301,
        "errno": 0,
        // TODO: Implement
    });
}

pub fn room_data_get(json: &serde_json::Value, state: &AppState, sender: &Arc<Mutex<Sender>>) {
    let data = json!({
        "action": 1302,
        "errno": 0,
        "data": json["data"]
    });
}

pub fn room_data_update(json: &serde_json::Value, state: &AppState, sender: &Arc<Mutex<Sender>>) {
    let player_id = json["playerId"].as_i64().unwrap_or_default();
    let data = json!({
        "action": 1303,
        "errno": 0,
        "data": {
            "playerId": player_id,
            "data": json["data"],
        }
    });
}

pub fn room_info_get(json: &serde_json::Value, state: &AppState, sender: &Arc<Mutex<Sender>>) {
    let data = json!({
        "action": 1304,
        "errno": 0,
        "data": json["data"]
    });
}

pub fn room_info_update(json: &serde_json::Value, state: &AppState, sender: &Arc<Mutex<Sender>>) {
    let player_id = json["playerId"].as_i64().unwrap_or_default();
    let data = json!({
        "action": 1305,
        "errno": 0,
        "data": {
            "playerId": player_id,
            // TODO: very nani
            "data": json["data"],
        }
    });
}

pub fn room_join(json: &serde_json::Value, state: &AppState, sender: &Arc<Mutex<Sender>>) {
    let data = json!({
        "action": 1306,
        "errno": 0,
        // TODO
        "data": ""
    });
}

pub fn room_kick(json: &serde_json::Value, state: &AppState, sender: &Arc<Mutex<Sender>>) {
    let executor_id = json["executorId"].as_i64().unwrap_or_default();
    let player_id = json["playerId"].as_i64().unwrap_or_default();
    let data = json!({
        "action": 1307,
        "errno": 0,
        "data": {
            "executorId": executor_id,
            "playerId": player_id
        }
    });
}

pub fn room_leave(json: &serde_json::Value, state: &AppState, sender: &Arc<Mutex<Sender>>) {
    let player_id = json["playerId"].as_i64().unwrap_or_default();
    let data = json!({
        "action": 1308,
        "errno": 0,
        // TODO
        "data": {
            "playerId": player_id
        }
    });
}

pub fn room_list(json: &serde_json::Value, state: &AppState, sender: &Arc<Mutex<Sender>>) {
    // TODO
    let data = json!({
        "action": 1309,
        "errno": 0
    });
}

pub fn room_password(json: &serde_json::Value, state: &AppState, sender: &Arc<Mutex<Sender>>) {
    let player_id = json["playerId"].as_i64().unwrap_or_default();
    let password = json["password"].as_str().unwrap_or_default();
    let data = json!({
        "action": 1310,
        "errno": 0,
        "data": {
            "playerId": player_id,
            "password": password
        }
    });
}

pub fn room_remove(json: &serde_json::Value, state: &AppState, sender: &Arc<Mutex<Sender>>) {
    let data = json!({
        "action": 1311,
        "errno": 0
    });
}
