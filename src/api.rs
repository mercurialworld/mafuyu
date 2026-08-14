use axum::{response::IntoResponse, Json, Router};
use log::debug;
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub async fn health() -> impl IntoResponse {
    Json(json!({"status": "ok"}))
}

pub async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await.unwrap();
    debug!("listening on {}", listener.local_addr().unwrap());

    let _ = axum::serve(listener, app).await;
}
