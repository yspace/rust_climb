use super::*;

const COLLECTION_NAME: &'static str = "adv_user2";

pub async  fn run(db: mongodb::Database) -> Result<()> {

    // init_collection(db.clone()).await?;

    aggregate_ops(db.clone()).await?;

    Ok(())
}
async fn init_collection(db: mongodb::Database) -> Result<()> {
    let collection = db.collection::<Document>(COLLECTION_NAME);

    let docs = vec![
        doc! {   "name": "张三" ,"date":"2023/3/4", "gender":1 , "score": 50},
        doc! {   "name": "莉丝" ,"date":"2023/3/4", "gender":1 , "score": 52},
        doc! {   "name": "王武" ,"date":"2023/3/4", "gender":1 , "score": 40},
        doc! {   "name": "张三" ,"date":"2023/3/1", "gender":1 , "score": 50},
        doc! {   "name": "宋江" ,"date":"2023/3/2", "gender":1 , "score": 60},
        doc! {   "name": "张飞" ,"date":"2023/3/3", "gender":1 , "score": 56},
    ];

    // Insert some documents into the "mydb.books" collection.
    collection.insert_many(docs, None).await?;

    Ok(())
}


async fn aggregate_ops(db: mongodb::Database) -> Result<()> {
    println!("enter aggregate_ops") ;
    let collection = db.collection::<Document>(COLLECTION_NAME);
    // 聚合操作 中间可以有0个或者多个阶段 每个阶段都可以用一个json文档来指定
    let pipeline = vec![ 
           doc!{"$match":{"gender":{"$eq":1}}},
           // name 去重
          doc!{"$group":{
             "_id":"$name",
             "doc_count": {"$sum":1},
             
             "max_score": {"$max":"$score"},
             "min_score": {"$min":"$score"},
             "avg_score": {"$avg":"$score"},
             "sum_score": {"$sum":"$score"},
            
            
             }},
         ];
    let options = None;
    let result = collection.aggregate(pipeline, options).await?;

    super::_printing_documents(result).await?;

    println!("end aggregate_ops");
    Ok(())
}
async fn __tpl(db: mongodb::Database) -> Result<()> {
    let collection = db.collection::<Document>("adv_person");


    Ok(())
}