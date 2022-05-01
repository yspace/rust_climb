use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use sqlx::PgConnection;

use std::net::TcpListener;

use crate::routes::*;

pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    // Wrap the connection in a smart pointer
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            // Middlewares are added using the `wrap` method on `App`
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    // .bind("127.0.0.1:8000")?
    // .bind(address)?
    .listen(listener)?
    .run();

    Ok(server)
}
