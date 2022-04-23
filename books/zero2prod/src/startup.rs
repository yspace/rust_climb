use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

use std::net::TcpListener;

use crate::routes::* ;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    // .bind("127.0.0.1:8000")?
    // .bind(address)?
    .listen(listener)?
    .run();

    Ok(server)
}
