mod db;
mod handlers;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use db::DB;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // shared application state
    let state = Arc::new(Mutex::new(DB::new()));

    // build router
    let app: Router<> = build_router(state);

    // start server
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind address");

    println!("🚀 Server running on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("server failed");
}

fn build_router(state: Arc<Mutex<DB>>) -> Router {
    Router::new()
        .route("/get/{key}", get(handlers::get_key))
        .route("/set/{key}", post(handlers::set_key))
        .route("/update/{key}", put(handlers::update_key))
        .route("/delete/{key}", delete(handlers::delete_key))
        .with_state(state)
}
