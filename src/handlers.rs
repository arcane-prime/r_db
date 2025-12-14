use std::sync::{Arc, Mutex};
use axum::{
    extract::{Path, State},
    response::Json,
};

use crate::db::DB;

pub async fn get_key(
    State(db): State<Arc<Mutex<DB>>>,
    Path(key): Path<String>,
) -> Json<Option<String>> {
    let db = db.lock().unwrap();
    Json(db.get(&key).cloned())
}

pub async fn set_key(
    State(db): State<Arc<Mutex<DB>>>,
    Path(key): Path<String>,
    Json(value): Json<String>
) -> Json<&'static str> { 
    let mut db = db.lock().unwrap();
    db.set(key, value);
    Json("OK")
}

pub async fn update_key(
    State(db): State<Arc<Mutex<DB>>>,
    Path(key): Path<String>,
    Json(value): Json<String>,
) -> Json<bool> {
    let mut db = db.lock().unwrap();
    Json(db.update(&key, value))
}

pub async fn delete_key(
    State(db): State<Arc<Mutex<DB>>>,
    Path(key): Path<String>,
) -> Json<Option<String>> {
    let mut db = db.lock().unwrap();
    Json(db.delete(&key))
}