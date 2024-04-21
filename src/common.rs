use axum::extract::ws::{Message, WebSocket};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, Mutex},
};
pub type WSSender = Arc<tokio::sync::Mutex<futures_util::stream::SplitSink<WebSocket, Message>>>;

pub struct User {
    pub id: i32,
    pub name: String,
    pub sender: WSSender,
}

pub struct Room {
    pub id: i32,
    pub users: BTreeSet<i32>,
}

pub struct AppState {
    pub online_users: Mutex<BTreeMap<i32, User>>,
    pub rooms: Mutex<Vec<Room>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            online_users: Mutex::new(BTreeMap::new()),
            rooms: Mutex::new(Vec::new()),
        }
    }
}
