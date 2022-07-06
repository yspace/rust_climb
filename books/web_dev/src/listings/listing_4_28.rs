use warp::{
    filters::cors::CorsForbidden, http::Method, http::StatusCode, reject::Reject, Filter,
    Rejection, Reply,
};

use crate::models::*;
use crate::store::Store;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}

#[derive(Debug)]
enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
}
impl Reject for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ParseError(ref err) => write!(f, "Parse error: {}", err),
            Error::MissingParameters => write!(f, "Missing parameters"),
        }
    }
}

async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", params);

    let mut start = 0;
    if let Some(n) = params.get("start") {
        start = n.parse::<usize>().expect("Cound not parse start");
    }
    println!("{:?}", start);

    let res: Vec<Question> = store.questions.values().cloned().collect();
    Ok(warp::reply::json(&res))
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}
fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::ParseError)?,
        });
    }
    Err(Error::MissingParameters)
}

pub async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    println!("{}", "Starting server...");
    let cors = warp::cors()
        .allow_any_origin()
        // .allow_header("content-type")
        .allow_header("not-in-the-request")
        .allow_methods(&[Method::PUT, Method::DELETE]);

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(get_questions)
        .recover(return_error);

    let routes = get_items.with(cors);
    println!("{} at : {}", "Starting server...", 3030);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
