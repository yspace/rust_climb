pub mod error;
pub mod web;

pub use self::error::{Error,Result};

use axum::handler::HandlerWithoutStateExt;
use axum::middleware;
use axum::response::Response;
use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};
use std::net::SocketAddr;
use tower_http::{services::ServeDir, trace::TraceLayer};

use serde::Deserialize;

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            "example_static_file_server=debug,tower_http=debug",
        )
    }
    tracing_subscriber::fmt::init();

    async fn handle_404() -> (StatusCode, &'static str) {
        (StatusCode::NOT_FOUND, "Not found")
    }

    let serve_dir = ServeDir::new("assets").not_found_service(handle_404.into_service());
    // build our application with a route

    let app = Router::new()
        .merge(hello_router())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        // .fallback_service(routes_static());
        .nest_service("/assets", serve_dir.clone())
        .fallback_service(serve_dir)
        .layer(TraceLayer::new_for_http());

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn main_response_mapper(res: Response) -> Response{
    println!("-->> {:<12} - main_response_mapper","RES_MAPPER");
   
    println!();
   res
}



fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new(".")))
}

fn hello_router() -> Router {
    let app = Router::new()
        .route("/hello", get(handler))
        .route("/hello2/:name", get(handler2));
    return app;
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler(Query(params): Query<HelloParams>) -> impl IntoResponse /*Html<&'static str>*/ {
    println!("->> {:<12} - handler_hello - {:?}", "HANDLER", params);

    let name = params.name.as_deref().unwrap_or("world!");
    let out = format!("<h1>Hello, {}!</h1>", name);
    Html(out)
}
async fn handler2(Path(name): Path<String>) -> impl IntoResponse /*Html<&'static str>*/ {
    println!("->> {:<12} - handler_hello - {:?}", "HANDLER", name);

    // let name =  name.as_deref().unwrap_or("world!");
    let out = format!("<h1>Hello, {}!</h1>", name);
    Html(out)
}
