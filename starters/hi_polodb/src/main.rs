use polodb_core::Database;
use polodb_core::bson::{ doc};

// 😢https://github.com/PoloDB/PoloDB/blob/master/src/polodb_core/coll/collection.rs 这东西是个trait
// 定义了集合香相关的操作中！不导入 方法就找不到
use polodb_core::CollectionT;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    title: String,
    author: String,
}


fn main()  -> Result<(), Box<dyn std::error::Error>>{

    // use polodb_core::Database;

    let db = Database::open_path("test-polo.db").unwrap();
    //


    // let db = Database::open_memory()?;
    let collection = db.collection::<Book>("books");

    let docs = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string() },
        Book { title: "Animal Farm".to_string(), author: "George Orwell".to_string() },
        Book { title: "The Great Gatsby".to_string(), author: "F. Scott Fitzgerald".to_string() },
    ];
    collection.insert_many(docs)?;

    let books = collection.find(doc! {}).run()?;
    for book in books {
        println!("name: {:?}", book);
    }

    println!("Hello, world!");

    Ok(())
}
