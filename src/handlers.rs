use axum::{
    extract::{Path, State},
    response::Json,
};
use std::sync::Arc;

use crate::state::AppState;

pub async fn get_key(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
) -> Json<Option<String>> {
    let db = state.db.lock().unwrap();
    Json(db.get(&key).cloned())
}

pub async fn set_key(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
    Json(value): Json<String>,
) -> Json<&'static str> {

    {
        let mut aof = state.aof.lock().unwrap();
        aof.write_set(&key, &value).unwrap();
    }

    {
        let mut db = state.db.lock().unwrap();
        db.set(key, value);
    }

    Json("OK")
}

pub async fn update_key(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
    Json(value): Json<String>,
) -> Json<bool> {

    {
        let mut aof = state.aof.lock().unwrap();
        aof.write_set(&key, &value).unwrap();
    }

    let mut db = state.db.lock().unwrap();
    Json(db.update(&key, value))
}

pub async fn delete_key(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
) -> Json<Option<String>> {

    {
        let mut aof = state.aof.lock().unwrap();
        aof.write_del(&key).unwrap();
    }

    let mut db = state.db.lock().unwrap();
    Json(db.delete(&key))
}
