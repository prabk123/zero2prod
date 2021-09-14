use crate::routes::{health_check, subscribe};
use actix_web::middleware::Logger;
use actix_web::{dev::Server, web, App, HttpServer};
use env_logger::Env;
use sqlx::PgPool;
use std::io::Error;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
