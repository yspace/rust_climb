// use self::models::*;
use db_diesel::models::*;
use diesel::prelude::*;
use db_diesel::*;

fn main() {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(1))
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}