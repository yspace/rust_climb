use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    // SqliteConnection 在prelude里面
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use self::models::{NewPost, Post};
use diesel::sql_types::{Integer, Double};
// 诡异的用法 @see https://docs.diesel.rs/diesel/macro.sql_function.html    
sql_function!(
    #[sql_name = "last_insert_rowid"]
    fn last_insert_rowid() -> Integer);


// no_arg_sql_function!(
//     last_insert_rowid,
//     diesel::sql_types::Integer,
//     "Represents the SQL last_insert_row() function"
// );

pub fn create_post(conn: &mut SqliteConnection, title: &str, body: &str) /* -> Post */ {
    use crate::schema::posts;

    let new_post = NewPost { title, body , published: &0};

    // 此处文档上的是针对pg 的其他数据库不支持returning的功能 
    diesel::insert_into(posts::table)
        .values(&new_post)
        // .get_result(conn)
        .execute(conn)
        .expect("Error saving new post");

    //SQLite supports RETURNING as of version 3.35. 
    let result = diesel::select(last_insert_rowid())
    .get_result::<i32>(conn);

    if !result.is_err() {

        println!("last insert rowid is {}", result.unwrap()) ;
    }

    // ===    
    let results = posts::table
    .filter(posts::dsl::title.like(format!("%{}%",new_post.title)))
    .load::<Post>(conn)
    .expect("Error getting new post");

    for result in results {
        println!("{:?}", result);
    }
}