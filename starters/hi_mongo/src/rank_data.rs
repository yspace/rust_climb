use super::*;

pub async fn run(db: mongodb::Database) -> Result<()> {
    // init_collection(db.clone()).await?;

    // and_ops(db.clone()).await?;
    // or_ops(db.clone()).await?;
    // aggregate_ops(db.clone()).await?;
    rank(db.clone()).await?;

    Ok(())
}

async fn init_collection(db: mongodb::Database) -> Result<()> {
    let collection = db.collection::<Document>("user_rank");

    let docs = vec![
        doc! { "user_id":10001, "score":3.5 },
        doc! { "user_id":10002, "score":3.6 },
        doc! { "user_id":10003, "score":3.7 },
        doc! { "user_id":10004, "score":3.8 },
        doc! { "user_id":10005, "score":3.9 },
    ];

    // Insert some documents into collection.
    collection.insert_many(docs, None).await?;

    Ok(())
}

use redis::AsyncCommands;

async fn rank(db: mongodb::Database) -> Result<()> {
    let collection = db.collection::<Document>("user_rank");

    let query_doc = doc! {};
    // 排序 1 升 -1降
    let find_options = FindOptions::builder()
        // projection 可以指定那些字段需要 那些不需要
        .projection(doc! {"_id":0})
        // 倒序
        .sort(doc! { "score": -1 })
        .build();

    let mut results = collection.find(query_doc, find_options).await?;

    // super::_printing_documents(results).await?;
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_async_connection().await?;

    // while let Some(item) = results.try_next().await? {
    //     // NOTE: Document 是bson 每个字段的获取需要知道其类型 可以先用debug方式打印下 观察其结构 然后在用对应的方法来获取字段
    //     // println!("{:?} {:?}", item.get_i32("user_id"),item.get_f64("score"));
    //    let _ = con.zadd("rank", item.get_i32("user_id")?, item.get_f64("score")?).await?;
    // }
    // con.zincr(key, member, delta) //  修改

 
    // 带 `withscore` 的方法返回结果单项元素就是一个元祖 。元祖第一个元素是值 第二个元素是评分
    let items: Vec<String> = con.zrangebyscore("rank", 3.4, 3.7).await?;
    // 不带rev的是从小到大排序
    // zrange系列 是按照索引位置取的 0是第一位置 1是第二位置 以此类推 range参数里面就是索引区间 -1是最后一个位置 -2则是倒数第二 ... 
    // let items: Vec<(String, f32)> = con.zrevrange_withscores("rank", 0, -1).await?;
    // 最大score是5 最小是-1 ，偏移从0开始 取最多两个元素 ；这种貌似跟分页有关
    let items: Vec<(String, f32)> = con.zrevrangebyscore_limit_withscores("rank", 5, -1,0,2).await?;
    println!("{:?}", items);
    let count: isize = con.zcard("rank").await.expect("zcard operation failed");
    println!("zcard : {:?}", count); // used to find the number of items in the sorted set

        let key = "rank";

    // 查看排名
    let x: isize = con.zrank("rank", 10004).await?;
        println!("1000排名: {:?}", x);
    // 值不存在则返回null , 所以最好用Option<xxx> 来应对这种情况
    let n: Option<isize> = con.zrank("rank",10009).await?;
    println!("10009: {:?}", n);

    // 》查看评分
    let s: Option<f32> = con.zscore(key, 10004).await?;
    println!("10004: score {:?}",s) ;
    // 》给定评分上下限范围内的元素数量
    let c: usize = con.zcount(key , 0,3.5).await?;
    println!("zcount {:?}",c) ;

    Ok(())
}
