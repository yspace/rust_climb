use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

use std::net::TcpListener;

// async fn greet(req: HttpRequest) -> impl Responder {
//     // You can only use impl Trait if your function returns a single type;
//     //  if you want to return multiple, you need dynamic dispatch
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

async fn health_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}
#[derive(serde::Deserialize)] struct FormData {
    email: String,
    name: String
}
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse { HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(
        || 
        App::new()
        .route("/health_check", web::get().to(health_check))
        .route("/subscriptions", web::post().to(subscribe))
    )
        // .bind("127.0.0.1:8000")?
        // .bind(address)?
        .listen(listener)?
        .run();

    Ok(server)
}
