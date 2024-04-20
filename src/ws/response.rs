use crate::common::AppState;
use axum::extract::ws::{Message, WebSocket};
use serde_json::json;

type Sender = futures_util::stream::SplitSink<WebSocket, Message>;

async fn send_json(sender: &mut Sender, json: serde_json::Value) {
    use futures_util::SinkExt;
    let message = Message::Text(serde_json::to_string(&json).unwrap());
    sender.send(message).await.unwrap();
}

pub async fn global_online_count(_json: &serde_json::Value, state: &AppState, sender: &mut Sender) {
    let count = state.online_users.lock().unwrap().len();
    let data = json!({
        "action": 1000,
        "errno": 0,
        "data": count
    });
    send_json(sender, data).await;
}

pub async fn match_end(json: &serde_json::Value, state: &AppState, sender: &mut Sender) {
    let data = json!({
        "action": 1100,
        "errno": 0
    });
}

pub async fn match_ready(json: &serde_json::Value, state: &AppState, sender: &mut Sender) {
    let data = json!({
        "action": 1101,
        "errno": 0
    });
}

pub async fn match_start(json: &serde_json::Value, state: &AppState, sender: &mut Sender) {
    let seed = json["seed"].as_i64().unwrap_or_default();
    let data = json!({
        "action": 1102,
        "errno": 0,
        "data": {
            "seed": seed
        }
    });
}

pub async fn player_config(json: &serde_json::Value, state: &AppState, sender: &mut Sender) {
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

pub async fn player_finish(json: &serde_json::Value, state: &AppState, sender: &mut Sender) {
    let data = json!({
        "action": 1201,
        "errno": 0,
        "data": {
            "playerId": 123,
            "data": json["data"]
        }
    });
}

pub async fn player_group(json: &serde_json::Value, state: &AppState, sender: &mut Sender) {
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

pub async fn player_ready(json: &serde_json::Value, state: &AppState, sender: &mut Sender) {
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

pub async fn player_role(json: &serde_json::Value, state: &AppState, sender: &mut Sender) {
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

pub async fn player_state(json: &serde_json::Value, state: &AppState, sender: &mut Sender) {
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

pub async fn player_stream(json: &serde_json::Value, state: &AppState, sender: &mut Sender) {
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

pub async fn player_type(json: &serde_json::Value, state: &AppState, sender: &mut Sender) {
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

pub async fn room_chat(json: &serde_json::Value, state: &AppState, sender: &mut Sender) {
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

pub async fn room_create(json: &serde_json::Value, state: &AppState, sender: &mut Sender) {
    let data = json!({
        "action": 1301,
        "errno": 0,
        // TODO: Implement
    });
}

pub async fn room_data_get(json: &serde_json::Value, state: &AppState, sender: &mut Sender) {
    let data = json!({
        "action": 1302,
        "errno": 0,
        "data": json["data"]
    });
}

pub async fn room_data_update(json: &serde_json::Value, state: &AppState, sender: &mut Sender) {
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

pub async fn room_info_get(json: &serde_json::Value, state: &AppState, sender: &mut Sender) {
    let data = json!({
        "action": 1304,
        "errno": 0,
        "data": json["data"]
    });
}

pub async fn room_info_update(json: &serde_json::Value, state: &AppState, sender: &mut Sender) {
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

pub async fn room_join(json: &serde_json::Value, state: &AppState, sender: &mut Sender) {
    let data = json!({
        "action": 1306,
        "errno": 0,
        // TODO
        "data": ""
    });
}

pub async fn room_kick(json: &serde_json::Value, state: &AppState, sender: &mut Sender) {
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

pub async fn room_leave(json: &serde_json::Value, state: &AppState, sender: &mut Sender) {
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

pub async fn room_list(json: &serde_json::Value, state: &AppState, sender: &mut Sender) {
    // TODO
    let data = json!({
        "action": 1309,
        "errno": 0
    });
}

pub async fn room_password(json: &serde_json::Value, state: &AppState, sender: &mut Sender) {
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

pub async fn room_remove(json: &serde_json::Value, state: &AppState, sender: &mut Sender) {
    let data = json!({
        "action": 1311,
        "errno": 0
    });
}
