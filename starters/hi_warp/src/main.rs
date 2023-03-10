use aqs::stores::Store;
use warp;
use warp::http::Method;
use warp::Filter;

mod aqs;

#[tokio::main]
async fn main() {


    // let _ = aqs::async_http_request::main().await;
    // return ;
    
    let store = Store::new();

    let store_filter = warp::any().map(move || store.clone());
    
    let cors = warp::cors()
        //如两国通商 
        .allow_any_origin() // 允许哪区域
        .allow_header("content-type") // 允许携带啥
        .allow_header("not-in-the-request")
        // 允许什么叫痛工具呀 飞机 还是轮船 还是火车
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter)
        .and_then(aqs::handlers::get_questions)
        .recover(aqs::handlers::return_error)
       ;

    let routes = get_items
     // browser intercepts a PUT request, for example, and sends an OPTION request first
     .with(cors);
    // start the server and pass the route filter to it
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

