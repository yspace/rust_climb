use aqs::handlers::{add_question, get_questions, return_error};
use aqs::stores::Store;
use warp;
use warp::http::Method;
use warp::Filter;

mod aqs;

#[tokio::main]
async fn main() {
    // let _ = aqs::async_http_request::main().await;
    // return ;

    env_logger::init();
    log::error!("This is an error!");
    log::info!("This is info!");
    log::warn!("This is a warning!");

    let log = warp::log::custom(|info| {
        eprintln!(
            "{} {} {} {:?} from {} with {:?}",
            info.method(),
            info.path(),
            info.status(),
            info.elapsed(),
            info.remote_addr().unwrap(),
            info.request_headers()
        );
    });
    let log = warp::log::custom(|info| {
        log::info!(
            "{} {} {} {:?} from {} with {:?}",
            info.method(),
            info.path(),
            info.status(),
            info.elapsed(),
            info.remote_addr().unwrap(),
            info.request_headers()
        );
    });

    let store = Store::new();

    let store_filter = warp::any().map(move || store.clone());

    let id_filter = warp::any().map(|| uuid::Uuid::new_v4().to_string());

    let cors = warp::cors()
        //如两国通商
        .allow_any_origin() // 允许哪区域
        .allow_header("content-type") // 允许携带啥
        .allow_header("not-in-the-request")
        // 允许什么交通工具呀 飞机 还是轮船 还是火车
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and(id_filter)
        .and_then(aqs::handlers::get_questions);
    // .recover(aqs::handlers::return_error);

    let add_question = warp::post()
        //
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(add_question);

    let routes = get_questions
        .or(add_question)
        .with(cors)
        .with(log)
        .recover(return_error);

    // start the server and pass the route filter to it
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
