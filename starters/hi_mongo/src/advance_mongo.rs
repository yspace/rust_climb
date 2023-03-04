use super::*;

pub async  fn run(db: mongodb::Database) -> Result<()> {

    init_collection(db.clone()).await?;

    // and_ops(db.clone()).await?;
    // or_ops(db.clone()).await?;
    aggregate_ops(db.clone()).await?;

    Ok(())
}

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

    // Insert some documents into the "mydb.books" collection.
    collection.insert_many(docs, None).await?;

    Ok(())
}
async fn and_ops(db: mongodb::Database) -> Result<()> {
    let collection = db.collection::<Document>("adv_person");

    let query_doc = doc! { "gender": 1};
    // 排序 1 升 -1降
    let find_options = FindOptions::builder().sort(doc! { "age": 1 }).build();

    let results = collection.find(query_doc, find_options.clone()).await?;
    // println!("results: {:?}", results);
    super::_printing_documents(results).await?;

    let query_doc = doc! {"age":{"$gt":20}, "gender":0};
    // 上面这个是隐式 and 下面是显式and条件
    let query_doc = doc! {"$and":[{"age":{"$gt":20}}, {"gender":0}]};
    let mix_query_doc = doc! {"age":{"$lt":100},  "$and":[{"age":{"$gt":20}}, {"gender":0}] };
    let results = collection.find(mix_query_doc, find_options).await?;
    // println!("results: {:?}", results);
    super::_printing_documents(results).await?;

    Ok(())
}

// or 有短路逻辑？！！！
async fn or_ops(db: mongodb::Database) -> Result<()> {
    let collection = db.collection::<Document>("adv_person");
    let query_doc = doc! { "$or":[ {"age": {"$gt":20}}, {"age": {"$lt":20}} ]};
    // 排序 1 升 -1降
    let find_options = FindOptions::builder()
    // projection 可以指定那些字段需要 那些不需要
    .projection(doc! {"_id":0})
    .sort(doc! { "age": 1 }).build();

    let results = collection.find(query_doc, find_options).await?;
    // println!("results: {:?}", results);
    super::_printing_documents(results).await?;

    Ok(())
}
// 嵌套子文档查询
async fn sub_document_query(db: mongodb::Database) -> Result<()> {
    let collection = db.collection::<Document>("adv_person");

    // 嵌套数组可以用索引查询 ： 查询“price”第1个数据大于500的所有记录
    let query_doc = doc!{
        "price.0":{"$gt": 500}
    };
    // 使用`.` 点号表达嵌套 层级关系
    let find_options = FindOptions::builder()
    .projection(doc!{})
    .sort(doc! { "age": 1 })
    .build();

    Ok(())
}

async fn aggregate_ops(db: mongodb::Database) -> Result<()> {
    println!("enter aggregate_ops") ;
    let collection = db.collection::<Document>("adv_person");
    // 聚合操作 中间可以有0个或者多个阶段 每个阶段都可以用一个json文档来指定
    let pipeline = vec![ 
        // match 阶段 跟find 接受的参数要求是一样的 
        doc!{"$match":{"age":{"$gt":10}, "gender":0 }},
        // project 是投影的意思 可以缩小范围 也可以放大 就像你用手电筒照墙壁一样 影子大小可以通过调焦缩放
        // doc!{"$project":{"_id":0 , "title":1, "name":1, "gender":1 ,"added_key":"some_value","added_key2":"$name"}},
        // 修改已有字段 name的输出; 最后一个字段也是用此技巧 拉平嵌套文档中的某字段
        doc!{"$project":{"_id":0 , "title":1, "name":"yaaaa!", "gender":1 ,"added_key":"some_value",
        "added_key2":"$name","profile_rich":"$profile.rich"
        ,"some_name_contain_doller_sign":{"$literal":"$normal_string"}
    }},
        // doc!{"$project":{"new_k":"any_value" }},
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