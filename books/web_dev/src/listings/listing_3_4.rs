use warp::Filter ;

use crate::models::* ;
use std::str::FromStr ;

async fn get_questions() -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question::new( 
        QuestionId::from_str("1").expect("No id provided"), "First Question".to_string(),
        "Content of question".to_string(), Some(vec!("faq".to_string())),
        );
        Ok(warp::reply::json(  &question
        ))
}

pub async fn main() {
    let get_items =
     warp::get()
    .and(warp::path("questions")) 
    .and(warp::path::end())
     .and_then(get_questions);
let routes = get_items;
warp::serve(routes)
 .run(([127, 0, 0, 1], 3030))
  .await;
}
