// use self::models::*;
use db_diesel::models::*;
use diesel::prelude::*;
use db_diesel::*;

fn main() {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = find_all(connection,Params{
        page:Some(0),
        page_size:Some(2),
    });

    let (vec_post, total_pages) = results.unwrap() ;

    println!("total pages: {:?}", total_pages);
    for  post in vec_post  {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }

    // println!("Displaying {} posts", results.len());
    // for post in results {
    //     println!("{}", post.title);
    //     println!("-----------\n");
    //     println!("{}", post.body);
    // }
}