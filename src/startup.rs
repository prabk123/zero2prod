use actix_web::{dev::Server, web, App, HttpServer, Result};
use std::io::Error;
use std::net::TcpListener;
use crate::routes::{health_check, subscribe};

pub fn run(listener: TcpListener) -> Result<Server, Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}