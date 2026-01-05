use axum::Router;
use tokio::net::TcpListener;
use tracing_subscriber::fmt::init;

mod api;
mod bitcoin;
mod domain;
mod security;
mod config;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    init();

    let app = Router::new()
        .merge(api::routes());

    let listener = TcpListener::bind("0.0.0.0:9000")
        .await
        .expect("Failed to bind port");

    println!("🚀 Linkbit Bitcoin Escrow Service running on 0.0.0.0:9000");

    axum::serve(listener, app)
        .await
        .unwrap();
}
