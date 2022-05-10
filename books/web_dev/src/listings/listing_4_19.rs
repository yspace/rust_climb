use warp::{
    http::StatusCode, http::Method ,
    filters::{
        cors::CorsForbidden,
    } , reject::Reject, Filter, Rejection, Reply};


use crate::models::*;
use crate::store::Store;
use std::str::FromStr;

#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}

async fn get_questions(store: Store) -> Result<impl warp::Reply, warp::Rejection> {
     let res: Vec<Question> = store.questions.values().cloned().collect();
     Ok(warp::reply::json(&res))
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<CorsForbidden>(){
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    }
    else {
       Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

pub async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());


    println!("{}", "Starting server...");
    let cors = warp::cors()
    .allow_any_origin()
    // .allow_header("content-type")
    .allow_header("not-in-the-request")
    .allow_methods(&[ Method::PUT, Method::DELETE]);

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(get_questions)
        .recover(return_error);

    let routes = get_items.with(cors);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
