use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;
pub mod pagination;

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


// 这个是废弃的方法
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
     // 获取到id msql 做法
    //  let last_insert_id = diesel::sql_query("select LAST_INSERT_ID() as id")
    //  .load::<Sequence>(conn).expect("get_id_error").first().unwrap().id;
    // ===    
    let results = posts::table
    .filter(posts::dsl::title.like(format!("%{}%",new_post.title)))
    .load::<Post>(conn)
    .expect("Error getting new post");

    for result in results {
        println!("{:?}", result);
    }
}


// ----------


#[derive(Debug)]
pub struct Params {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    // ..
}
pub fn find_all(conn: &mut SqliteConnection,params: Params) -> Result<(Vec<Post>, i64),Box<dyn std::error::Error>> {
    use crate::schema::posts;

    use crate::pagination::Paginate;
    
    let mut query = posts::table.into_boxed();
    // ..

    let (posts, total_pages) = match params.page {
        Some(page) => {
            //  let q_count = query.paginate(page) ;
            //  let  total_infos: (Vec<Post>, i64) = q_count.load_and_count_pages(conn)? ;
            //  println!("total info:   {}"  ,total_infos.1) ;

            //  use the provided .load_and_count_pages() instead of .load()


             let res = query.paginate(page).load::<(Post, i64)>(conn)?;

            let total = res.get(0).map(|x| x.1).unwrap_or(0);
           
            println!("total: {:?}", total);

            let items = res.into_iter().map(|x| x.0).collect();
            let total_pages = (total as f64 / 10 as f64).ceil() as i64;
             
            (items, total_pages)
        },
        None => (query.load(conn)?, 1),
    };
    
    Ok((posts, total_pages))
}