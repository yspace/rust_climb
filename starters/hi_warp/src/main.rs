use warp;
use warp::Filter;

mod aqs;

#[tokio::main]
async fn main() {

    // let _ = aqs::async_http_request::main().await;
    // return ;

    let hello = 
    warp::get()
    .map(|| format!("Hello, World!"));
    warp::serve(hello).run(([127, 0, 0, 1], 8080)).await;
}
