use axum::{
    extract::{Path, State},
    response::Json,
    http::StatusCode,
};
use std::sync::Arc;

use crate::state::AppState;
use serde_json::Value;
use axum::response::Result;


pub async fn get_data(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
) -> Result<Json<Option<Value>>, (StatusCode, String)> {
    let storage_manager = state.storage_manager.lock().unwrap();
    let result = storage_manager
        .get(&key);
    
    Ok(Json(result))
}

pub async fn set_data(
    State(state): State<Arc<AppState>>,
    Json(value): Json<Value>,
) -> Result<Json<String>, (StatusCode, String)> { 
    let mut storage_manager = state.storage_manager.lock().unwrap();
    let key = storage_manager
        .put(value)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(key))
}

// pub async fn update_key(
//     State(state): State<Arc<AppState>>,
//     Path(key): Path<String>,
//     Json(value): Json<String>,
// ) -> Json<bool> {

//     {
//         let mut aof = state.aof.lock().unwrap();
//         aof.write_set(&key, &value).unwrap();
//     }

//     let mut db = state.db.lock().unwrap();
//     Json(db.update(&key, value))
// }

// pub async fn delete_key(
//     State(state): State<Arc<AppState>>,
//     Path(key): Path<String>,
// ) -> Json<Option<String>> {

//     {
//         let mut aof = state.aof.lock().unwrap();
//         aof.write_del(&key).unwrap();
//     }

//     let mut db = state.db.lock().unwrap();
//     Json(db.delete(&key))
// }
