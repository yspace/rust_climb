use bson::Document;
use mongodb::{options::ClientOptions, Client};

use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let mut options = ClientOptions::parse("mongodb://localhost:27017").await?;
    options.app_name = Some("My App".to_string());

    // Creating a Client
    let client = Client::with_options(options)?;

    // 上面两步如果不需要额外配置可以归为一步： let client = Client::with_uri_str("mongodb://mongodb0.example.com:27017").await?;
    // Client uses std::sync::Arc internally 所以可以通过clone方法 安全的在多线程和async 任务上被共享

    let db = client.database("my_db1");

    // for i in 0..5 {
    //     let db_ref = db.clone();

    //     task::spawn(async move {
    //         let collection = db_ref.collection::<Document>(&format!("coll{}", i));

    //         // Do something with the collection
    //     });
    // }
    let collection = db.collection::<Document>(&format!("users"));

    // Query the books in the collection with a filter and an option.
    let filter = doc! { "name": "yiqing" };
    let find_options = FindOptions::builder().sort(doc! { "age": 1 }).build();
    let mut cursor = collection.find(filter, find_options).await?;

    // Iterate over the results of the cursor.
    while let Some(book) = cursor.try_next().await? {
        // println!("title: {}", book.address);
        println!("{:?}",book) ;
    }

    Ok(())
}
