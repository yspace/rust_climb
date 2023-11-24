use std::fs::File;

use actix_web::{error::Error, web, App, HttpResponse, HttpServer};
async fn hello0() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hello there!"))
}

async fn hello() -> Result<HttpResponse, Error> {
    let _ = File::open("fictionalfile.txt")?;
    Ok(HttpResponse::Ok().body("File read successfully"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/hello", web::get().to(hello)))
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
