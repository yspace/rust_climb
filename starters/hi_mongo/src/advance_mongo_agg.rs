use mongodb::options::DropCollectionOptions;

use super::*;

const COLLECTION_NAME: &'static str = "adv_user2";

pub async  fn run(db: mongodb::Database) -> Result<()> {

    // init_collection(db.clone()).await?;

    // aggregate_ops(db.clone()).await?;
    // array_unwind(db.clone()).await?;
    join_ops(db.clone()).await?;

    Ok(())
}
async fn init_collection(db: mongodb::Database) -> Result<()> {
    let collection = db.collection::<Document>(COLLECTION_NAME);

    let docs = vec![
        doc! {   "name": "张三" ,"date":"2023/3/4", "gender":1 , "score": 20},
        doc! {   "name": "莉丝" ,"date":"2023/3/4", "gender":1 , "score": 82},
        doc! {   "name": "王武" ,"date":"2023/3/4", "gender":1 , "score": 40},
        doc! {   "name": "张三" ,"date":"2023/3/1", "gender":1 , "score": 90},
        doc! {   "name": "宋江" ,"date":"2023/3/2", "gender":1 , "score": 60},
        doc! {   "name": "张飞" ,"date":"2023/3/3", "gender":1 , "score": 76},
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
            
             // 分组内取第一 最后插入的就是最新数据 取最老的就是last的对立面 $first 可以做`去重后取最早插入的记录`
             "date": {"$last":"$date"},
             "score": {"$last":"$score"},
            
             }},
         ];
    let options = None;
    let result = collection.aggregate(pipeline, options).await?;

    super::_printing_documents(result).await?;

    println!("end aggregate_ops");
    Ok(())
}


// 数组展开 类似数据表中的join操作  展开这个词很容易想到堆栈展开
async fn array_unwind(db: mongodb::Database) -> Result<()> {
    let collection_name = "adv_products";
    let collection = db.collection::<Document>(collection_name);

    let docs = vec![
        doc! {   "name": "衬衣" ,"size":["S","M","L","XL"],"price":[100,200,300,400]},
        doc! {   "name": "长裤" ,"size":["S","M","L","XL"],"price":[100,200,300,400]},
        doc! {   "name": "长裙" ,"size":["S","M","L","XL"],"price":[100,200,300,400]},
        doc! {   "name": "短裙" ,"size":["S","M","L","XL"],"price":[100,200,300,400]},
        doc! {   "name": "风衣" ,"size":["S","M","L","XL"],"price":[100,200,300,400]},
        doc! {   "name": "睡衣" ,"size":["S","M","L","XL"],"price":[100,200,300,400]},
    ];

    // Insert some documents into the "mydb.books" collection.
    collection.insert_many(docs, None).await?;

    // 查询
    let pipeline = vec![ 
        // 根据size字段来展开
       doc!{"$unwind": "$size"},
       // 由于数组展开一次只能一个字段 所以对price字段的第二次展开作为另一个stage阶段
       doc!{"$unwind": "$price"},
      ];
    let options = None;
    let result = collection.aggregate(pipeline, options).await?;
    super::_printing_documents(result).await?;

    // 清空集合
    let drop_opt = DropCollectionOptions::builder()
    .build();
    collection.drop(drop_opt).await?;
    Ok(())
}

async fn join_ops(db: mongodb::Database) -> Result<()> {
    let collection_name = "weibo_users";
    let collection = db.collection::<Document>(collection_name);

    let docs = vec![
        doc! { "id":1001,  "name": "小王" , "register_date":"2005-06-08","age":16,"work":"学生"},
        doc! { "id":1002,  "name": "小李" , "register_date":"2005-04-08","age":26,"work":"公务员"},
        doc! { "id":1003,  "name": "小张" , "register_date":"2005-06-08","age":36,"work":"土匪"},
        doc! { "id":1004,  "name": "小赵" , "register_date":"2005-07-08","age":46,"work":"工程师"},
        doc! { "id":1005,  "name": "小楚" , "register_date":"2005-09-08","age":56,"work":"秘书"},
        doc! { "id":1006,  "name": "小卫" , "register_date":"2005-01-08","age":66,"work":"老师"},
    ];
    // Insert some documents into the "mydb.books" collection.
    collection.insert_many(docs, None).await?;
    // 第二个集合的创建
    let collection_name = "weibo_posts";
    let collection_posts = db.collection::<Document>(collection_name);

    let docs = vec![
        doc! { "user_id":1001,  "content": "小王来啦" , "post_time":"2005-06-08 12:23:12"},
        doc! { "user_id":1001,  "content": "小王在吃饭呢" , "post_time":"2005-06-08 12:08:12"},
        doc! { "user_id":1001,  "content": "小王要睡觉啦！" , "post_time":"2005-22-08 22:23:12"},
        doc! { "user_id":1001,  "content": "小王来啦" , "post_time":"2005-06-08 12:23:12"},
       
        doc! { "user_id":1002,  "content": "小李在吃饭呢" , "post_time":"2005-06-08 12:08:12"},
        doc! { "user_id":1002,  "content": "小李要睡觉啦！" , "post_time":"2005-22-08 22:23:12"},
        doc! { "user_id":1003,  "content": "小李来啦" , "post_time":"2005-06-08 12:23:12"},
        doc! { "user_id":1004,  "content": "小李在吃饭呢" , "post_time":"2005-06-08 12:08:12"},

        doc! { "user_id":1003,  "content": "小张要睡觉啦！" , "post_time":"2005-22-08 22:23:12"},
        doc! { "user_id":1003,  "content": "小张来啦" , "post_time":"2005-06-08 12:23:12"},
        doc! { "user_id":1003,  "content": "小张在吃饭呢" , "post_time":"2005-06-08 12:08:12"},
        doc! { "user_id":1003,  "content": "小张要睡觉啦！" , "post_time":"2005-22-08 22:23:12"},

         
    ];
    collection_posts.insert_many(docs, None).await?;

    // 联集合查询
    let pipeline = vec![ 
        doc!{"$lookup":{
            "from":"weibo_users",
            "localField":"user_id",
            "foreignField":"id",
            // 注意查询结果中此字段是数组形态出现的
            "as":"user_info",
        }},
        // 第二步 展开数组
        doc!{"$unwind": "$user_info"},
        // 第三步 提取特定字段 
        doc!{"$project":{
            "content":1,
            "post_time":1,
            // "name":"$user_info.name",
            // "work":"$user_info.work",
        }},
      ];
    let options = None;
    // 注意主集合 顺序不要搞反了 有点类似关系型数据库中的连表查询 left join 所以以谁做基表比较重要
    // TODO ：也可以以用户集合为主 查询用户发送的微博列表 然后展开 再提取特定字段｜或者重命名 ；如果想查询特定用户的微博列表 可以添加`$match`阶段
    let result = collection_posts.aggregate(pipeline, options).await?;
    super::_printing_documents(result).await?;

    // 清空集合
    let drop_opt = DropCollectionOptions::builder()
    .build();
    collection.drop(drop_opt).await?;
    collection_posts.drop(None).await?;
    Ok(())
}
async fn __tpl(db: mongodb::Database) -> Result<()> {
    let collection = db.collection::<Document>("adv_person");


    Ok(())
}