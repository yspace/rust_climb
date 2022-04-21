use actix_web::{
    web, App, HttpRequest, HttpServer, 
    Responder,HttpResponse
    
};

async fn greet(req: HttpRequest) -> impl Responder {
    // You can only use impl Trait if your function returns a single type;
    //  if you want to return multiple, you need dynamic dispatch
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

async fn health_check(req: HttpRequest) -> impl Responder {
  HttpResponse::Ok().finish()
}


pub async fn run() -> std::io::Result<()> {
   
    HttpServer::new(|| {
        App::new()
        .route("/health_check", web::get().to(health_check))
        .route("/", web::get().to(greet))
        .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await 
}
