mod db;
mod handlers;
mod aof;
mod state;

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use state::AppState;
use aof::Aof;
use db::DB;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
    let mut db = DB::new();
    Aof::replay("data.aof", &mut db).expect("AOF replay failed");
    
    // shared application state
    let state = Arc::new(AppState {
        db: Mutex::new(db),
        aof: Mutex::new(Aof::new("data.aof").unwrap()),
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
        .route("/get/{key}", get(handlers::get_key))
        .route("/set/{key}", post(handlers::set_key))
        .route("/update/{key}", put(handlers::update_key))
        .route("/delete/{key}", delete(handlers::delete_key))
        .with_state(state)
}
