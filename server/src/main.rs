use actix::*;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::time::Instant;

mod chat;
mod global_mods;
mod media;
mod media_manager;
mod room;
mod server;
mod session;
mod user;
use dotenv::dotenv;
use server::Server;
use session::WsSession;
use std::env;

/// Entry point for user's websocket route
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<Server>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WsSession {
            id: 0,
            session_id: 0,
            rng: rand::thread_rng(),
            hb: Instant::now(),
            room_addr: None,
            room: None,
            server_addr: srv.get_ref().clone(),
            is_login: false,
        },
        &req,
        stream,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let server_addr: String = env::var("SERVER_ADDR").unwrap();
    let server_port: String = env::var("SERVER_PORT").unwrap();
    let arb = Arbiter::new();
    let server_actor_addr = arb
        .exec(move || async move {
            let server = Server::new().await;
            server.start()
        })
        .await
        .unwrap()
        .await;
    // Create Http server with websocket support
    HttpServer::new(move || {
        App::new()
            .data(server_actor_addr.clone())
            .service(web::resource("/ws/").to(chat_route))
    })
    .bind(server_addr + ":" + server_port.as_str())
    .unwrap()
    .run()
    .await
}
