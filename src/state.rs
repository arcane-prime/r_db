use std::sync::Mutex;
use crate::{db::DB, aof::Aof};

pub struct AppState {
    pub db: Mutex<DB>,
    pub aof: Mutex<Aof>,
}