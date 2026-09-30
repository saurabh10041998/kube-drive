#![allow(unused)]

use axum::routing::method_routing::get;
use axum::response::Html;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let routes_ns = axum::Router::new().route(
        "/api/v1/namespaces",
        get(|| async { Html("kubectl get <strong>namespace</strong>")})
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    println!("->> LISTENING ON {listener:?}");

    axum::serve(listener, routes_ns)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install ctrl+c handler");
    println!("->> Shutting down..");
}