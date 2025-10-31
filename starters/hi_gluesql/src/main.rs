 use gluesql::prelude::*;

// @see https://github.com/gluesql/gluesql/blob/main/pkg/rust/examples/hello_world.rs
use {


    std::fs,
};
 use gluesql::prelude::{Glue, FileStorage};

async fn _main() {
    let storage = FileStorage::new("data/mutable-api").unwrap();
    let mut glue = Glue::new(storage);

    let sqls = [
        "CREATE TABLE Glue (id INTEGER);",
        "INSERT INTO Glue VALUES (100);",
        "INSERT INTO Glue VALUES (200);",
        "DROP TABLE Glue;",
    ];

    for sql in sqls {
        let output = glue.execute(sql).await.unwrap();
        println!("{:?}", output)
    }
}

 use futures;

 fn main() {
     futures::executor::block_on(_main());
 }