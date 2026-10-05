use axum::{routing::get, Json, Router};
use serde_json::{json, Value};
use std::net::SocketAddr;

async fn index() -> &'static str {
    "Hello from Rust (axum) on Naijacloud!\n"
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

#[tokio::main]
async fn main() {
    // Naijacloud passes the port your service listens on as PORT.
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health));

    // Bind 0.0.0.0, not 127.0.0.1, so traffic from outside the container reaches it.
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await
        .unwrap();
}
