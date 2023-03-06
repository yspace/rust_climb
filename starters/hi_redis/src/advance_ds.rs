use derive_redis_json::RedisJsonValue;
use redis::RedisResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, RedisJsonValue, Debug)]
pub struct UserInfo {
    pub age: usize,
    pub salary: f64,
    pub address: String,
}

pub fn run(mut con: &mut redis::Connection) -> redis::RedisResult<()> {
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
    let is_online: bool = con.hexists(key, field)?;
    if is_online {
        println!("user:{} is online", field);
    } else {
        println!("user:{} is offline", field);
    }

    // 下线用户
    let _ = con.hdel(key, field)?;
    if !con.hexists(key, field)? {
        println!("user:{} is not online", field);
    }
    // =======
    for i in 0..1000 {
        con.hset(key, format!("user_{}", i), 1)?;
    }
    let all_online_users: Vec<String> = con.hgetall(key)?;
    println!("all_online_users:{:?}", all_online_users);

    Ok(())
}
fn usecase_hash2(con: &mut redis::Connection) -> redis::RedisResult<()> {
    use redis::Commands; // 命令的高级方式（以方法名出现的那些命令）在此traits中实现

    let key = "user_info";
    let field = "user_001";

    let user_info = UserInfo {
        age: 17,
        address: "xi'an".to_string(),
        salary: 3000.0,
    };

    let _ = con.hset(key, field, user_info)?;

    let is_online: bool = con.hexists(key, field)?;
    if is_online {
        println!("user:{} is online", field);
    } else {
        println!("user:{} is offline", field);
    }

    let items = vec![(
        "user_002",
        UserInfo {
            age: 30,
            salary: 3000.0,
            address: "beijing".into(),
        },
    )];

    con.hset_multiple(key, &items)?;

    let users: Vec<(String, UserInfo)> = con.hgetall(key)?;

    for (attr, user_info) in users.iter() {
        println!("user{}:{:?}", attr, user_info);
    }

    // 
    let field_names:Vec<String> = con.hkeys(key)?;
    println!("field_names: {:?}", field_names); 

    let info : UserInfo = con.hget(key,"user_002")?;
    println!("info: {:?}", info);

    // let info_list : Vec<(String,UserInfo)> = con.hgetall(key)?;
    // TODO： 这里的类型实际是多样的 也可以是任意一种符合条件的类型
    let info_list : Vec<(String,UserInfo)> =  redis::cmd("hmget")
    .arg(key)
    .arg(vec![
        "user_002",
        "user_001",
    ])
    .query(con)?;
for (attr, user_info) in info_list.iter() {
    println!("user>>{}:{:?}", attr, user_info);
}
   // 等价做法 hget 内部的实现竟然可以用hmget
   let info_list : Vec<UserInfo> = con.hget(key, vec![
    "user_002",
    "user_001",
   ])?;
   for info in info_list.iter() {
     println!("user info {:?}", info);
   }
  
  println!("hget 特殊情况");
  // hget 在key 或者field 不存在时都返回null
  let result: RedisResult<Option<String>> = con.hget(format!("{}{}",key,"not_exist"), field);
     println!("not exist: {:?}",result);
  let result: RedisResult<bool> = con.hexists(format!("{}{}",key,"not_exist"), field);
     println!("is exist: {:?}",result);

   // 查询哈希 字段数目
   let field_nums : f64 = con.hlen(key)?; 
   // 实际上返回值的类型可以大概给！！！比如本例子可以给 f64,i64,i32... 都可以， 因为是一个trait：FromRedisValue
   println!("field nums: {:?}",field_nums);

    Ok(())
}
