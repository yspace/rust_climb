use warp::{
    http::StatusCode, http::Method ,
    filters::{
        cors::CorsForbidden,
    } , reject::Reject, Filter, Rejection, Reply};


use crate::models::*;
use std::str::FromStr;

#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}

async fn get_questions() -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );

    match question.id.0.is_empty() {
        true => Err(warp::reject::custom(InvalidId)),
        false => Ok(warp::reply::json(&question)),
    }
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
    println!("{}", "Starting server...");
    let cors = warp::cors()
    .allow_any_origin()
    // .allow_header("content-type")
    .allow_header("not-in-the-request")
    .allow_methods(&[ Method::PUT, Method::DELETE]);

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions)
        .recover(return_error);

    let routes = get_items.with(cors);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
