#[macro_use]
extern crate diesel;

extern crate dotenv;

use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;

mod schema;
mod models;

fn main() {
    // dotenv().ok();
    dotenv().expect("cannot find the env file");
    
    for (key, _value) in env::vars() {
        // println!("{}: {}", key, value);
        println!("{}", key);
    }

    // let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    let database_url = String::from("postgres://postgres:yiqing@localhost/api_dev");
    let conn = PgConnection::establish(&database_url).unwrap();

    let book = models::NewBook {
        title: String::from("Gravity's Rainbow"),
        author: String::from("Thomas Pynchon"),
        published: true,
    };

    if models::Book::insert(book, &conn) {
        println!("success");
    } else {
        println!("failed");
    }
}