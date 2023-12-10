use actix::*;
use actix_web::{
    web::{self, Data},
    App, Error, HttpRequest, HttpResponse, HttpServer,
};
use actix_web_actors::ws;
use app_infrastructure::{
    app_config::{AppConfiguration, AppConfigurationBuilder},
    app_tracing, BoxError,
};
use serde::Deserialize;
use std::{net::SocketAddr, time::Instant};
use tracing::info;

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

#[derive(Deserialize)]
struct HttpSettings {
    address: SocketAddr,
}

#[actix_web::main]
async fn main() -> Result<(), BoxError> {
    dotenv()?;
    let AppConfiguration {
        app_environment: _,
        config,
    } = AppConfigurationBuilder::default().build()?;
    app_tracing::init_from_config(&config)?;

    let http_settings = config.get::<HttpSettings>("http")?;
    let server_actor_addr = Server::new().await.start();
    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(server_actor_addr.clone()))
            .service(web::resource("/ws/").to(chat_route))
    })
    .bind(&http_settings.address)?;

    info!("Server started at {}", http_settings.address);
    server.run().await?;
    Ok(())
}
