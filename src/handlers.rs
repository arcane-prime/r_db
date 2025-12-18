use axum::{
    extract::{Path, State},
    response::Json,
};
use std::sync::Arc;

use crate::state::AppState;
use serde_json::Value;


pub async fn get_data(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
) -> Json<Option<Value>> { 
    let storage_manager = state.storage_manager.lock().unwrap();
    Json(storage_manager.get(&key))
}

pub async fn set_data(
    State(state): State<Arc<AppState>>,
    Path(key): Path<String>,
    Json(value): Json<Value>,
) -> Json<&'static str> { 
    let storage_manager = state.storage_manager.lock().unwrap();
    let p = storage_manager.put(key, value);
    
    Json("Ok")
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
