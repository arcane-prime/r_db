mod handlers;
mod aof;
mod state;
mod storage;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use state::AppState;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use dotenvy::dotenv;
use crate::storage::StorageManager;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let storage_manager = StorageManager::new()
    .expect("Failed to initialize StorageManager and perform AOF recovery.");

    // shared application state
    let state = Arc::new(AppState {
        storage_manager: Mutex::new(storage_manager)
    });

    // build router
    let app = build_router(state);

    // start server
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind address");

    println!("🚀 Server running on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("server failed");
}

fn build_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/get/{key}", get(handlers::get_data))
        .route("/set", post(handlers::set_data))
        // .route("/update/{key}", put(handlers::update_key))
        // .route("/delete/{key}", delete(handlers::delete_key))
        .with_state(state)
}
