
// use axum_sqlite::*;

use axum::{Extension, response::{IntoResponse,Html}};
use r2d2::{Pool, Error, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
// https://github.com/programatik29/tokio-rusqlite 这个应该也可以使用

#[derive(Clone)]
pub struct Database {
    pool: Pool<SqliteConnectionManager>,
}

impl Database {
    pub fn new(path: &str) -> Result<Extension<Self>, Error> {
        let manager = SqliteConnectionManager::file(path);
        let pool = Pool::new(manager)?;
        Ok(Extension(Self { pool }))
    }

    pub fn connection(
        &self,
    ) -> Result<PooledConnection<SqliteConnectionManager>, Error> {
        Ok(self.pool.get()?)
    }
}

pub fn init_db()->  Result<Extension<Database>, Error>{
    let path = ":memory:" ;
    /*
      dbg!("Initialising sqlite database");
      let sql_schema = include_str!("schema.sql");
      conn.execute(sql_schema, [])?; 
     */
    return Database::new(path)
}


// ==

struct Person {     
    name: String,
    place: String,
}
// Extension(database): Extension<db::Database>
pub async  fn create(Extension(db): Extension<Database> /*, Json(p): Json<Person> */) -> impl IntoResponse {
    println!("create begin") ;
  
    let conn = db.connection().unwrap(); // Do stuff with connection
    
    // let conn = connection.
   conn
    .execute("CREATE TABLE IF NOT EXISTS foo (bar INTEGER)", params![])
    .unwrap();

    println!("created! done!");

    // conn.execute(
    //     "INSERT INTO profile (name, place) VALUES (?1, ?2)",
    //     params![&p.name, &p.place],
    // );
}