// lib.rs

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use tracing;
use actix_files::Files;


type MessageMap = Arc<Mutex<std::collections::HashMap<String, String>>>;

#[derive(Deserialize, Serialize)]
struct SendMessage {
    user: String,
    message: String,
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn post_message(
    send_message: web::Json<SendMessage>,
    message_map: web::Data<MessageMap>,
) -> HttpResponse {
    let mut map = message_map.lock().unwrap();
    map.insert(send_message.user.clone(), send_message.message.clone());

    // Log the received message
    tracing::info!(
        "Received message from {}: {}",
        send_message.user,
        send_message.message
    );

    HttpResponse::Ok().body(format!(
        "Message from {}: {}",
        send_message.user, send_message.message
    ))
}

async fn read_messages(message_map: web::Data<MessageMap>) -> HttpResponse {
    let map = message_map.lock().unwrap();

    let messages: Vec<String> = map
        .iter()
        .map(|(user, message)| format!("{}: {}", user, message))
        .collect();

    // Log the messages being read
    tracing::info!("Messages read: {:?}", messages);

    HttpResponse::Ok().json(messages)
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let message_map: MessageMap = Arc::new(Mutex::new(std::collections::HashMap::new()));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(message_map.clone()))
            .route("/health_check", web::get().to(health_check))
            .route("/send", web::post().to(post_message))
            .route("/read", web::get().to(read_messages))
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
