#[macro_use]
extern crate actix_web;
use actix_web::{middleware, web, App, HttpRequest, HttpServer, Result};
use serde::Serialize;

pub struct MessageApp {
    port: u16 ,
}

impl MessageApp{
    pub fn new(port: u16) -> Self{
        MessageApp{port}
    }

    pub fn run(&self) -> std::io::Result<()> {
        //  &self : The calling code also maintains ownership so
        //we are just borrowing the instance.

        println!("Starting http server: 127.0.0.1:{}", self.port) ;

        HttpServer::new(move || {
            /*
            If the keyword move comes before the argument list then any variables from the
environment that the closure uses are actually moved into the closure. This means
the closure takes ownership of those variables rather than creating references.
This implies that the lifetime of the closure can be longer can its surrounding
environment because those variables are moved into the closure. Without the move
keyword, variables closed over are actually just references to the surrounding
environment.
*/
            App::new()
                .wrap(middleware::Logger::default())
                .service(index)
        })
            .bind(("127.0.0.1", self.port))?
            .workers(8)
            .run()
    }
}

#[derive(Serialize)]
struct IndexResponse {
    message: String,
}

#[get("/")]
fn index(req: HttpRequest) -> Result<web::Json<IndexResponse>> {
    let hello = req
        .headers()
        .get("hello")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_else(|| "world");
    Ok(web::Json(IndexResponse {
        message: hello.to_owned(),
    }))
}