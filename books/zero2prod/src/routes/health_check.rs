use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
pub async fn health_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}
