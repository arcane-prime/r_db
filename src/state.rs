use std::sync::Mutex;
use crate::{ storage::StorageManager};

pub struct AppState {
    pub storage_manager: Mutex<StorageManager>
}