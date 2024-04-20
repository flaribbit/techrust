use axum::extract::ws::{Message, WebSocket};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, HashSet},
    sync::{Arc, Mutex},
};

type Sender = futures_util::stream::SplitSink<WebSocket, Message>;

struct Room {
    id: i32,
    users: HashSet<i32>,
}

pub struct AppState {
    rooms: Mutex<Vec<Room>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            rooms: Mutex::new(Vec::new()),
        }
    }
}

pub fn global_online_count(json: &serde_json::Value, state: &Arc<AppState>, sender: &Sender) {
    let n = 0;
    let data = json!({
        "action": 1000,
        "errno": 0,
        "data": n
    });
}

pub fn match_end(json: &serde_json::Value, state: &Arc<AppState>, sender: &Sender) {
    let data = json!({
        "action": 1100,
        "errno": 0
    });
}

pub fn match_ready(json: &serde_json::Value, state: &Arc<AppState>, sender: &Sender) {
    let data = json!({
        "action": 1101,
        "errno": 0
    });
}

pub fn match_start(json: &serde_json::Value, state: &Arc<AppState>, sender: &Sender) {
    let seed = json["seed"].as_i64().unwrap_or_default();
    let data = json!({
        "action": 1102,
        "errno": 0,
        "data": {
            "seed": seed
        }
    });
}

pub fn player_config(json: &serde_json::Value, state: &Arc<AppState>, sender: &Sender) {
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

pub fn player_finish(json: &serde_json::Value, state: &Arc<AppState>, sender: &Sender) {
    let data = json!({
        "action": 1201,
        "errno": 0,
        "data": {
            "playerId": 123,
            "data": json["data"]
        }
    });
}

pub fn player_group(json: &serde_json::Value, state: &Arc<AppState>, sender: &Sender) {
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

pub fn player_ready(json: &serde_json::Value, state: &Arc<AppState>, sender: &Sender) {
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

pub fn player_role(json: &serde_json::Value, state: &Arc<AppState>, sender: &Sender) {
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

pub fn player_state(json: &serde_json::Value, state: &Arc<AppState>, sender: &Sender) {
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

pub fn player_stream(json: &serde_json::Value, state: &Arc<AppState>, sender: &Sender) {
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

pub fn player_type(json: &serde_json::Value, state: &Arc<AppState>, sender: &Sender) {
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

pub fn room_chat(json: &serde_json::Value, state: &Arc<AppState>, sender: &Sender) {
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

pub fn room_create(json: &serde_json::Value, state: &Arc<AppState>, sender: &Sender) {
    let data = json!({
        "action": 1301,
        "errno": 0,
        // TODO: Implement
    });
}

pub fn room_data_get(json: &serde_json::Value, state: &Arc<AppState>, sender: &Sender) {
    let data = json!({
        "action": 1302,
        "errno": 0,
        "data": json["data"]
    });
}

pub fn room_data_update(json: &serde_json::Value, state: &Arc<AppState>, sender: &Sender) {
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

pub fn room_info_get(json: &serde_json::Value, state: &Arc<AppState>, sender: &Sender) {
    let data = json!({
        "action": 1304,
        "errno": 0,
        "data": json["data"]
    });
}

pub fn room_info_update(json: &serde_json::Value, state: &Arc<AppState>, sender: &Sender) {
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

pub fn room_join(json: &serde_json::Value, state: &Arc<AppState>, sender: &Sender) {
    let data = json!({
        "action": 1306,
        "errno": 0,
        // TODO
        "data": ""
    });
}

pub fn room_kick(json: &serde_json::Value, state: &Arc<AppState>, sender: &Sender) {
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

pub fn room_leave(json: &serde_json::Value, state: &Arc<AppState>, sender: &Sender) {
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

pub fn room_list(json: &serde_json::Value, state: &Arc<AppState>, sender: &Sender) {
    // TODO
    let data = json!({
        "action": 1309,
        "errno": 0
    });
}

pub fn room_password(json: &serde_json::Value, state: &Arc<AppState>, sender: &Sender) {
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

pub fn room_remove(json: &serde_json::Value, state: &Arc<AppState>, sender: &Sender) {
    let data = json!({
        "action": 1311,
        "errno": 0
    });
}
