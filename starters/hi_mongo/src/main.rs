use bson::Document;
use mongodb::{options::ClientOptions, Client};

use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions};

mod advance_mongo;
mod advance_mongo_agg;
mod advance_mongo_index;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let mut options = ClientOptions::parse("mongodb://localhost:27017").await?;
    options.app_name = Some("My App".to_string());

    // Creating a Client
    let client = Client::with_options(options)?;

    // 上面两步如果不需要额外配置可以归为一步： let client = Client::with_uri_str("mongodb://mongodb0.example.com:27017").await?;
    // Client uses std::sync::Arc internally 所以可以通过clone方法 安全的在多线程和async 任务上被共享

    let db = client.database("my_db1");

    // ######  
    // advance_mongo::run(db.clone()).await?;
    // advance_mongo_agg::run(db.clone()).await?;
    advance_mongo_index::run(db.clone()).await?;
    return Ok(());


    // _inserting_documents_into_a_collection(db.clone()).await?;
    _updating_documents_into_a_collection(db.clone()).await?;

    // for i in 0..5 {
    //     let db_ref = db.clone();

    //     task::spawn(async move {
    //         let collection = db_ref.collection::<Document>(&format!("coll{}", i));

    //         // Do something with the collection
    //     });
    // }
    let collection = db.collection::<Document>(&format!("users"));

    // 删除文档的示例
    let query = doc! { "age":66};
    let delete_result = collection.delete_many(query, None).await;
    if delete_result.is_ok() {
        println!("delete OK: {:?}", delete_result.unwrap());
    } else {
        println!("delete Err: {:?}", delete_result.err());
    }

    // 去重 ； 如果相对满足特定条件的记录集合去重 需要构造第二个参数
    let distinct_result = collection.distinct("name", doc! {}, None).await?;
    println!("distinct OK: {:?}", distinct_result);
    let distinct_result = collection
        .distinct("name", doc! {"age":{"$gt": 20}}, None)
        .await?;
    println!("distinct OK: {:?}", distinct_result);

    // Query the books in the collection with a filter and an option.
    // let filter = doc! { "name": "yiqing" };
    let filter = doc! { "age": {"$gte": 19} };
    // 排序 1 升 -1降
    let find_options = FindOptions::builder().sort(doc! { "age": 1 }).build();
    let mut cursor = collection.find(filter, find_options).await?;

    // Iterate over the results of the cursor.
    while let Some(book) = cursor.try_next().await? {
        // println!("title: {}", book.address);
        println!("{:?}", book);
    }
    println!("=======");
    let mut cursor = collection.find(doc! {
        "id":null
    }, None).await?;
    _printing_documents(cursor).await? ;

    Ok(())
}

use mongodb::Cursor;

#[cfg(all(not(feature = "sync"), not(feature = "tokio-sync")))]
pub async fn _printing_documents(mut cursor:  Cursor<Document>) -> Result<()> {
    // use mongodb::bson::{doc, Document};
    // Iterate over the results of the cursor.

    println!("$$$ >> begin _printing_documents");
    while let Some(book) = cursor.try_next().await? {
        // println!("title: {}", book.address);
        println!("{:?}", book);
    }
println!("$$$ >> end _printing_documents");
    Ok(())
}
#[cfg(all(not(feature = "sync"), not(feature = "tokio-sync")))]
async fn _inserting_documents_into_a_collection(db: mongodb::Database) -> Result<()> {
    // use mongodb::bson::{doc, Document};

    // Get a handle to a collection in the database.
    let collection = db.collection::<Document>("users");

    let docs = vec![
        doc! { "title": "1984", "name": "George Orwell" ,"age":16},
        doc! { "title": "Animal Farm", "name": "George Orwell" , "age": 32},
        doc! { "title": "The Great Gatsby", "name": "F. Scott Fitzgerald" , "age":20},
    ];

    // Insert some documents into the "mydb.books" collection.
    collection.insert_many(docs, None).await?;

    Ok(())
}

#[cfg(all(not(feature = "sync"), not(feature = "tokio-sync")))]
async fn _updating_documents_into_a_collection(db: mongodb::Database) -> Result<()> {
    // use mongodb::bson::{doc, Document};

    // Get a handle to a collection in the database.
    let collection = db.collection::<Document>("users");

    let query = doc! { "title": "1984", "name": "George Orwell" ,"age":16};
    let update_doc = doc! { "$set": { "age":66 } };

    let docs = vec![
        doc! { "title": "1984", "name": "George Orwell" ,"age":16},
        doc! { "title": "Animal Farm", "name": "George Orwell" , "age": 32},
        doc! { "title": "The Great Gatsby", "name": "F. Scott Fitzgerald" , "age":20},
    ];

    // Insert some documents into the "mydb.books" collection.
    collection.update_many(query, update_doc, None).await?;

    Ok(())
}

// ========== 下面是一个错误处理的典型实现 ,也可以使用Enum哦 表示不同种类的错误===========
// #[derive(Clone, Debug, Error)] // use thiserror::Error;
#[derive(Clone, Debug)]
struct Err {
    db_error: mongodb::error::Error
}
impl From<mongodb::error::Error> for Err {
    fn from(error: mongodb::error::Error) -> Self {
        Err {db_error:error}
    }
}

impl std::error::Error for Err {}
impl std::fmt::Display for Err {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "({}, {})", self.x, self.y)
        write!(f, "some error happens! {}",self.db_error)
    }
}

#[allow(dead_code)]
type Result<T> = std::result::Result<T, Err>;
