use crate::common::{AppState, Room, WSSender};
use axum::extract::ws::Message;
use serde_json::{json, Value};

fn send_json(sender: &WSSender, json: &Value) {
    use futures_util::SinkExt;
    let sender: WSSender = sender.clone();
    let message = Message::Text(json.to_string());
    tokio::spawn(async move {
        match sender.lock().await.send(message).await {
            Ok(_) => println!("send json success"),
            Err(e) => println!("send json error: {}", e),
        }
    });
}

fn send_others(player_id: i32, state: &AppState, json: &Value) {
    let rooms = state.rooms.lock().unwrap();
    let online_users = state.online_users.lock().unwrap();
    rooms
        .iter()
        .find(|room| room.users.contains(&player_id))
        .map(|room| {
            room.users
                .iter()
                .filter(|id| **id != player_id)
                .for_each(|id| {
                    send_json(&online_users[id].sender, json);
                });
        });
}

fn send_room(player_id: i32, state: &AppState, json: &Value) {
    let rooms = state.rooms.lock().unwrap();
    let online_users = state.online_users.lock().unwrap();
    rooms
        .iter()
        .find(|room| room.users.contains(&player_id))
        .map(|room| {
            room.users.iter().for_each(|id| {
                send_json(&online_users[id].sender, json);
            });
        });
}

pub fn global_online_count(_json: &Value, state: &AppState, id: i32, sender: &WSSender) {
    let count = state.online_users.lock().unwrap().len();
    let data = json!({
        "action": 1000,
        "errno": 0,
        "data": count
    });
    send_json(sender, &data);
}

pub fn match_end(json: &Value, state: &AppState, id: i32, sender: &WSSender) {
    let data = json!({
        "action": 1100,
        "errno": 0
    });
    send_room(id, state, &data);
}

pub fn match_ready(json: &Value, state: &AppState, id: i32, sender: &WSSender) {
    let data = json!({
        "action": 1101,
        "errno": 0
    });
    send_room(id, state, &data);
}

pub fn match_start(json: &Value, state: &AppState, id: i32, sender: &WSSender) {
    let seed = json["seed"].as_i64().unwrap_or_default();
    let data = json!({
        "action": 1102,
        "errno": 0,
        "data": {
            "seed": seed
        }
    });
    send_room(id, state, &data);
}

pub fn player_config(json: &Value, state: &AppState, id: i32, sender: &WSSender) {
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

pub fn player_finish(json: &Value, state: &AppState, id: i32, sender: &WSSender) {
    let data = json!({
        "action": 1201,
        "errno": 0,
        "data": {
            "playerId": 123,
            "data": json["data"]
        }
    });
    send_others(id, state, &data);
}

pub fn player_group(json: &Value, state: &AppState, id: i32, sender: &WSSender) {
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

pub fn player_ready(json: &Value, state: &AppState, id: i32, sender: &WSSender) {
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

pub fn player_role(json: &Value, state: &AppState, id: i32, sender: &WSSender) {
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

pub fn player_state(json: &Value, state: &AppState, id: i32, sender: &WSSender) {
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

pub fn player_stream(json: &Value, state: &AppState, id: i32, sender: &WSSender) {
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

pub fn player_type(json: &Value, state: &AppState, id: i32, sender: &WSSender) {
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

pub fn room_chat(json: &Value, state: &AppState, id: i32, sender: &WSSender) {
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

pub fn room_create(json: &Value, state: &AppState, id: i32, sender: &WSSender) {
    let data = json!({
        "action": 1301,
        "errno": 0,
        // TODO: Implement
    });
}

pub fn room_data_get(json: &Value, state: &AppState, id: i32, sender: &WSSender) {
    let data = json!({
        "action": 1302,
        "errno": 0,
        "data": json["data"]
    });
}

pub fn room_data_update(json: &Value, state: &AppState, id: i32, sender: &WSSender) {
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

pub fn room_info_get(json: &Value, state: &AppState, id: i32, sender: &WSSender) {
    let data = json!({
        "action": 1304,
        "errno": 0,
        "data": json["data"]
    });
}

pub fn room_info_update(json: &Value, state: &AppState, id: i32, sender: &WSSender) {
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

pub fn room_join(json: &Value, state: &AppState, id: i32, sender: &WSSender) {
    let data = json!({
        "action": 1306,
        "errno": 0,
        // TODO
        "data": ""
    });
}

pub fn room_kick(json: &Value, state: &AppState, id: i32, sender: &WSSender) {
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

pub fn room_leave(json: &Value, state: &AppState, id: i32, sender: &WSSender) {
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

pub fn room_list(json: &Value, state: &AppState, id: i32, sender: &WSSender) {
    // TODO
    let data = json!({
        "action": 1309,
        "errno": 0
    });
}

pub fn room_password(json: &Value, state: &AppState, id: i32, sender: &WSSender) {
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

pub fn room_remove(json: &Value, state: &AppState, id: i32, sender: &WSSender) {
    let data = json!({
        "action": 1311,
        "errno": 0
    });
}
