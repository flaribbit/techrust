use serde_json::{json, Value};

pub fn global_online_count(n: i32) -> Value {
    json!({
      "action": 1000,
      "errno": 0,
      "data": n
    })
}

pub fn match_end() -> Value {
    json!({
      "action": 1100,
      "errno": 0
    })
}

pub fn match_ready() -> Value {
    json!({
      "action": 1101,
      "errno": 0
    })
}

pub fn match_start(seed: i32) -> Value {
    json!({
      "action": 1102,
      "errno": 0,
      "data": {
        "seed": seed
      }
    })
}

pub fn player_config(data: &str) -> Value {
    json!({
      "action": 1200,
      "errno": 0,
      "data": data
    })
}

pub fn player_config2(player_id: i32, data: &str) -> Value {
    json!({
      "action": 1200,
      "errno": 0,
      "data": {
        "playerId": player_id,
        "config": data
      }
    })
}

pub fn player_finish(data: &Value) -> Value {
    json!({
      "action": 1201,
      "errno": 0,
      "data": {
        "playerId": 123,
        "data": data
      }
    })
}

pub fn player_group(player_id: i32, group_id: i32) -> Value {
    json!({
      "action": 1202,
      "errno": 0,
      "data": {
        "playerId": player_id,
        "group": group_id
      }
    })
}

pub fn player_ready(player_id: i32, is_ready: bool) -> Value {
    json!({
      "action": 1203,
      "errno": 0,
      "data": {
        "playerId": player_id,
        "isReady": is_ready
      }
    })
}

pub fn player_role(player_id: i32, role: &str) -> Value {
    json!({
      "action": 1204,
      "errno": 0,
      "data": {
        "playerId": player_id,
        "role": role
      }
    })
}

pub fn player_state(player_id: i32, state: &str) -> Value {
    json!({
      "action": 1205,
      "errno": 0,
      "data": {
        "playerId": player_id,
        "customState": state
      }
    })
}

pub fn player_stream(player_id: i32, data: &str) -> Value {
    json!({
      "action": 1206,
      "errno": 0,
      "data": {
        "playerId": player_id,
        "stream": data
      }
    })
}

pub fn player_type(player_id: i32, player_type: &str) -> Value {
    json!({
      "action": 1207,
      "errno": 0,
      "data": {
        "playerId": player_id,
        "type": player_type
      }
    })
}

pub fn room_chat(player_id: i32, message: &str) -> Value {
    json!({
      "action": 1300,
      "errno": 0,
      "data": {
        "playerId": player_id,
        "message": message
      }
    })
}

pub fn room_create() -> Value {
    json!({
      "action": 1301,
      "errno": 0,
      // TODO: Implement
    })
}

pub fn room_data_get(data: &Value) -> Value {
    json!({
      "action": 1302,
      "errno": 0,
      "data": data
    })
}

pub fn room_data_update(player_id: i32, data: &Value) -> Value {
    json!({
      "action": 1303,
      "errno": 0,
      "data": {
        "playerId": player_id,
        "data": data,
      }
    })
}

pub fn room_info_get(data: &Value) -> Value {
    json!({
      "action": 1304,
      "errno": 0,
      "data": data
    })
}

pub fn room_info_update(player_id: i32, data: &Value) -> Value {
    json!({
      "action": 1305,
      "errno": 0,
      "data": {
        "playerId": player_id,
        // TODO: very nani
        "data": data,
      }
    })
}

pub fn room_join() -> Value {
    json!({
      "action": 1306,
      "errno": 0,
      // TODO
      "data": ""
    })
}

pub fn room_join2(player_id: i32, room_id: i32) -> Value {
    json!({
      "action": 1306,
      "errno": 0,
      // TODO
      "data": ""
    })
}

pub fn room_kick(executor_id: i32, player_id: i32) -> Value {
    json!({
      "action": 1307,
      "errno": 0,
      "data": {
        "executorId": executor_id,
        "playerId": player_id
      }
    })
}

pub fn room_leave() -> Value {
    json!({
      "action": 1308,
      "errno": 0
    })
}

pub fn room_leave2(player_id: i32) -> Value {
    json!({
      "action": 1308,
      "errno": 0,
      "data": {
        "playerId": player_id
      }
    })
}

pub fn room_list() -> Value {
    // TODO
    json!({
      "action": 1309,
      "errno": 0
    })
}

pub fn room_password(player_id: i32, password: &str) -> Value {
    json!({
      "action": 1310,
      "errno": 0,
      "data": {
        "playerId": player_id,
        "password": password
      }
    })
}

pub fn room_remove() -> Value {
    json!({
      "action": 1311,
      "errno": 0
    })
}
