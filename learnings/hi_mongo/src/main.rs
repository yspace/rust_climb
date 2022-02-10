use mongodb::{bson::doc, sync::Client};
// use bson::oid::ObjectId ;
use mongodb::bson::{oid::ObjectId, Bson};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    #[serde(rename = "_id")]
    id: ObjectId,
    title: String,
    author: String,
}

fn main() {
    match do_it() {
        Ok(_) => {
            println!("OK");
        }
        Err(e) => {
            println!("do_it Err: {:?}", e);
        }
    }
}

fn do_it() -> Result<(), mongodb::error::Error> {
    // let client = Client::with_uri_str("mongodb://localhost:27017")?;
    let client = Client::with_uri_str("mongodb://admin:password@localhost:27017")?;

    let database = client.database("mydb");
    let collection = database.collection::<Book>("books");

    let docs = vec![
        // Book {
        //     title: "1984".to_string(),
        //     author: "George Orwell".to_string(),
        // },
        // Book {
        //     title: "Animal Farm".to_string(),
        //     author: "George Orwell".to_string(),
        // },
        Book {
            id: ObjectId::new(),
            title: "The Great Gatsby".to_string(),
            author: "F. Scott Fitzgerald".to_string(),
        },
    ];

    // List the names of the collections in that database.
    for collection_name in database.list_collection_names(None)? {
        println!("collection: {}", collection_name);
    }
    // Insert some books into the "mydb.books" collection.
    //collection.insert_many(docs, None)?;

    // let cursor = collection.find(doc! { "author": "George Orwell" }, None)?;
    let cursor = collection.find(doc! {}, None)?;
    for result in cursor {
        // println!("title: {}", result?.title);
        println!("title: {:?}", result?);
    }

    Ok(())
}
