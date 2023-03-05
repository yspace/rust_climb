use mongodb::{options::{CreateIndexOptions, CreateIndexOptionsBuilder}, IndexModel};

use super::*;

pub async  fn run(db: mongodb::Database) -> Result<()> {

    init_collection(db.clone()).await?;

    // and_ops(db.clone()).await?;
    // or_ops(db.clone()).await?;
    // aggregate_ops(db.clone()).await?;

    Ok(())
}
// 关于mongodb索引 可以看这里 @see https://www.mongodb.com/docs/manual/indexes/
// 除了索引 也可以配合redis 来加速数据的处理
async fn init_collection(db: mongodb::Database) -> Result<()> {
    let collection = db.collection::<Document>("adv_person");

    let docs = vec![
        doc! { "title": "1984", "name": "George Orwell" ,"age":16, "gender":1 , "tags":["富有","白"], "profile":{"rich":1}},
        doc! { "title": "1984", "name": "George Orwell" ,"age":16, "gender":1 , "tags":["富有","白"], "profile":{"rich":1}},
        doc! { "title": "1984", "name": "George Orwell" ,"age":16, "gender":1 , "tags":["富有","白"], "profile":{"rich":1}},
        doc! { "title": "1984", "name": "George Orwell" ,"age":16, "gender":1 , "tags":["富有","白"], "profile":{"rich":1}},
       
        doc! { "title": "Animal Farm", "name": "George Orwell" , "age": 32 , "gender":0, "tags":["富有","矮小","猥琐"], "profile":{"rich":1}},
        doc! { "title": "Animal Farm", "name": "George Orwell" , "age": 32 , "gender":0, "tags":["富有","矮小","猥琐"], "profile":{"rich":1}},
        doc! { "title": "Animal Farm", "name": "George Orwell" , "age": 32 , "gender":0, "tags":["富有","矮小","猥琐"], "profile":{"rich":1}},
       
        doc! { "title": "The Great Gatsby", "name": "F. Scott Fitzgerald" , "age":20 , "gender":1, "tags":["穷","白","美"], "profile":{"rich":0}},
        doc! { "title": "The Great Gatsby", "name": "F. Scott Fitzgerald" , "age":20 , "gender":1, "tags":["穷","白","美"], "profile":{"rich":0}},
    ];

    // 创建索引
    let create_index_options = CreateIndexOptions::builder().build();
    let index_mode = IndexModel::builder()
    .keys(doc!{
        "name":-1,
    })
    .build();
    collection.create_index(index_mode, create_index_options).await?;
    let mut created_indexes = collection.list_indexes(None).await?;
    // for index in created_indexies {
    //     println!("{:?}", index);
    // }
    while let Some(idx) = &created_indexes.try_next().await? {
        // println!("title: {}", book.address);
        println!("index: {:?}", idx);
    }

    // Insert some documents into the "mydb.books" collection.
    collection.insert_many(docs, None).await?;

    Ok(())
}