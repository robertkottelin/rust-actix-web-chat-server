// main.rs

use chat::run;
use std::net::TcpListener;
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize the tracing subscriber
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let address = TcpListener::bind("127.0.0.1:8000")?;
    info!("Chat server started at 127.0.0.1:8000");

    run(address)?.await
}