use warp;
use warp::Filter;

mod aqs;

#[tokio::main]
async fn main() {
    // let _ = aqs::async_http_request::main().await;
    // return ;

    // create a path Filter
    let hello = warp::path("hello")
    .map(|| format!("Hello, World!"));
    // start the server and pass the route filter to it
    warp::serve(hello).run(([127, 0, 0, 1], 8080)).await;
}
