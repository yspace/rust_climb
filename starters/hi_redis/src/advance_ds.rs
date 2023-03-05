use serde::{Serialize, Deserialize};
use derive_redis_json::RedisJsonValue;


#[derive(Serialize, Deserialize, RedisJsonValue,Debug)]
pub struct UserInfo {
  pub age: usize,
  pub salary: f64,
  pub address: String,
}



pub fn  run(mut con: &mut redis::Connection) -> redis::RedisResult<()> {

    // usecase_hash(&mut con);
    usecase_hash2(&mut con);

    Ok(())
}

// @see https://lib.rs/crates/derive-redis-json
fn usecase_hash(con: &mut redis::Connection) -> redis::RedisResult<()> {
    use redis::Commands; // 命令的高级方式（以方法名出现的那些命令）在此traits中实现

    let key = "user_online_status";
    let field = "some_user_id";

    // 一个哈希表的Key里面可以设置成百上千个键值对
    let _ = con.hset(key, field, "somestring")?;
    // NOTE 必须处理result？ 不然说推断不出返回类型！！！
    let is_online: bool= con.hexists(key, field)?;
    if is_online {
        println!("user:{} is online", field);
    }else{
        println!("user:{} is offline", field);
    }

    // 下线用户
    let  _ = con.hdel(key, field)?;
    if !con.hexists(key, field)?{
        println!("user:{} is not online", field);
    }
    // =======
    for i in 0..1000{
        con.hset(key,format!("user_{}",i) , 1)?;
    }
    let all_online_users:Vec<String> = con.hgetall(key)?;
    println!("all_online_users:{:?}", all_online_users);

    Ok(())
}
fn usecase_hash2(con: &mut redis::Connection) -> redis::RedisResult<()> {
    use redis::Commands; // 命令的高级方式（以方法名出现的那些命令）在此traits中实现

    let key = "user_info";
    let field = "user_001";

    let user_info = UserInfo{
        age:17,
        address: "xi'an".to_string(),
        salary: 3000.0,
    };

    let _ = con.hset(key, field, user_info)?;

    let is_online: bool= con.hexists(key, field)?;
    if is_online {
        println!("user:{} is online", field);
    }else{
        println!("user:{} is offline", field);
    }

    let items = vec![
        ("user_002",UserInfo{age:30,salary:3000.0,address:"beijing".into()}),
    ];

    con.hset_multiple(key, &items)?;

    let users:Vec<(String,UserInfo)> = con.hgetall(key)?;

        for (attr,user_info) in users.iter() {
            println!("user{}:{:?}", attr, user_info);
        }   
    

    Ok(())
}