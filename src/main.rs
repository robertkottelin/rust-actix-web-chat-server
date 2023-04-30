use actix::{Actor, Handler, Message, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::sync::{Arc, Mutex};

type Clients = Arc<Mutex<Vec<actix::Addr<MyWs>>>>;

#[derive(Message)]
#[rtype(result = "()")]
struct TextMessage(String);

/// Define HTTP actor
struct MyWs {
    clients: Clients,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

impl MyWs {
    fn new(clients: Clients) -> Self {
        MyWs { clients }
    }
}

/// Handler for custom TextMessage
impl Handler<TextMessage> for MyWs {
    type Result = ();

    fn handle(&mut self, msg: TextMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                println!("Received message: {}", text);
                ctx.text(text)
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}


async fn index(clients: web::Data<Clients>, req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let clients = clients.get_ref().clone();
    let ws_actor = MyWs::new(clients);
    ws::start(ws_actor, &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let clients: Clients = Arc::new(Mutex::new(Vec::new()));
    let data = web::Data::new(clients);

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/ws/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
