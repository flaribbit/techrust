use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, Mutex},
};

pub struct User {
    pub id: i32,
    pub name: String,
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
