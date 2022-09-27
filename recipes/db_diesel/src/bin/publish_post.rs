// use self::models::Post;
use db_diesel::models::Post;
use diesel::prelude::*;
use db_diesel::*;
use std::env::args;

fn main() {
    use db_diesel::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut establish_connection();

    let _ = diesel::update(posts.find(id))
        .set(published.eq(1))
        // .get_result::<Post>(connection)
        .execute(connection)
        .expect(&format!("unable to find post {}", id) );

    println!("Published  " );
}