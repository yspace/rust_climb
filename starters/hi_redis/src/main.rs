use std::{num::NonZeroUsize, usize};

mod chat;
mod advance_ds;
mod pub_subs;
fn main() {
    println!("Hello, world!");

    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    // ## 
    advance_ds::run(&mut con).unwrap() ; return ();

    let result = do_something(&mut con);
    match result {
        Ok(_) => println!("ok!"),
        Err(err) => println!("error: {:?}", err),
    }
}

fn do_something(mut con: &mut redis::Connection) -> redis::RedisResult<()> {
    use redis::Commands;
   

    let _: () = con.set("key", "hello")?;
    let _: () = con.set("my_key", 42)?;
    let _: () = con.set("my_key2", 43)?;
    let _: () = con.set_nx("my_key", 41)?;
    let _: () = con.append("key", " redis")?;

    let k: Option<String> = con.get("key")?;
    /* do something here */
    println!("{:?}", k.unwrap());

    // let keys :  Vec<String> = con.keys("*").unwrap();
    // println!("{:?}", keys);

    println!("{:?}", do_something2(&mut con).unwrap());
    println!("{:?}", do_something3(&mut con).unwrap());

    // usecase_keys(&mut con)?;
    // usecase_user_id2names(&mut con)?;
    // usecase_list(&mut con)?;
    usecase_list_chat(&mut con)?;
    // usecase_sets(&mut con)?;

    Ok(())
}

fn do_something2(con: &mut redis::Connection) -> redis::RedisResult<usize> {
    // // This will result in a server error: "unknown command `MEMORY USAGE`"
    // // because "USAGE" is technically a sub-command of "MEMORY".
    // redis::cmd("MEMORY USAGE").arg("my_key").query(con)?;

    // However, this will work as you'd expect
    redis::cmd("MEMORY").arg("USAGE").arg("my_key").query(con)
}

fn do_something3(con: &mut redis::Connection) -> redis::RedisResult<usize> {
    use redis::Commands; // 命令的高级方式（以方法名出现的那些命令）在此traits中实现

    let key = "counter";
    let _ = con.set("counter", 1)?;
    let _ = con.incr("counter", 4)?;
    let _ = con.decr("counter", 3)?;

    let counter: usize = con.get(key)?;
    println!("counter: {}", counter);

    // However, this will work as you'd expect
    redis::cmd("MEMORY").arg("USAGE").arg("my_key").query(con)
}
fn usecase_keys(con: &mut redis::Connection) -> redis::RedisResult<()> {
    use redis::Commands; // 命令的高级方式（以方法名出现的那些命令）在此traits中实现

    let keys: Vec<String> = con.keys("*")?;
    println!("keys: {:?}", keys);

    let keys: Vec<String> = con.keys("my_*")?;
    println!("keys with my prefix: {:?}", keys);

    // However, this will work as you'd expect
    // redis::cmd("MEMORY").arg("USAGE").arg("my_key").query(con)
    Ok(())
}
fn usecase_user_id2names(con: &mut redis::Connection) -> redis::RedisResult<()> {
    use redis::Commands; // 命令的高级方式（以方法名出现的那些命令）在此traits中实现

    // redis 实战中 key的设计往往很有技巧性 需要考虑key不冲突 一般有特定模式  比如<project>:<usecase>:<...>
    let key_prefix = "user_id2name";

    let _ = con.set(format!("{}:{}", key_prefix, 100001), "yiqing")?;
    let _ = con.set(format!("{}:{}", key_prefix, 100002), "yiqing2")?;
    let _ = con.set(format!("{}:{}", key_prefix, 100003), "yiqing3")?;
    let _ = con.set(format!("{}:{}", key_prefix, 100004), "yiqing4")?;

    let keys: Vec<String> = con.keys("user_id2name:*")?;
    println!("keys with my prefix: {:?}", keys);

    let user_name: String = con.get(format!("{}:{}", key_prefix, 100001))?;
    println!("user_name: {:?}", user_name);

    // However, this will work as you'd expect
    // redis::cmd("MEMORY").arg("USAGE").arg("my_key").query(con)
    Ok(())
}

fn usecase_list(con: &mut redis::Connection) -> redis::RedisResult<()> {
    use redis::Commands; // 命令的高级方式（以方法名出现的那些命令）在此traits中实现

    let key = "l_somequeuename";
    // let _ = con.lpush("l_somequeuename",vec![1,3,4])? ;
    // let _ = con.rpush("l_somequeuename",vec![99,100])? ;

    let len_of_list: usize = con.llen(key)?;
    println!(" len of list: {:?}", len_of_list);

    // 索引访问 `lrange key 开始索引 结束索引` 当开始索引等于结束索引时就是精确到单个元素的获取了
    let items: Vec<usize> = con.lrange(key, 0, 0)?;
    println!(" items: {:?}", items);

    // 所有数据
    let items: Vec<usize> = con.lrange(key, 0, -1)?;
    println!(" items: {:?}", items);
    // NOTE: 不要贸然列举所有数据 如果是百万级别 会瞬间耗尽io资源的 。可以先看列表长度 然后可以利用正负索引来看头几个或者尾部几个元素。

    // 弹出数据
    // let val:usize = con.lpop(key, None)?;
    // println!("poped value: {}", val);
    // // 弹出数据
    // let vals:Vec<usize> = con.lpop(key, NonZeroUsize::new(5))?;
    // println!("poped values: {:?}", vals);

    // ## 修改数据
    // `lset key index 新值`
    let _ = con.lset(key, 0, 200)?;
    let val: Vec<usize> = con.lrange(key, 0, 0)?;
    println!("lset result: {:?}", val);

    // REDIS 列表可以用作队列
    // 用例： 节假日 给注册的几万用户发送祝福语 可以多台机器同时运行相同的发送逻辑

    Ok(())
}

fn usecase_list_chat(con: &mut redis::Connection) -> redis::RedisResult<()> {
    use crypto::digest::Digest;
    use crypto::md5::Md5;

    use redis::Commands; // 命令的高级方式（以方法名出现的那些命令）在此traits中实现

    let key = "l_chatlist";

    let len_of_list: usize = con.llen(key)?;
    println!(" len of list: {:?}", len_of_list);

    let user_id = 2 ;

    let msg = format!("{}","this could be a json string");

      // TODO 短期内不可以重复发送相同消息 `uid+msg_md5` 作为key 然后添加超时删除时间
    // 如果标准的md5算法太慢 可以选择更快的哈希实现 
    let mut hasher = Md5::new();
   
    let text =  format!("{}_{}", user_id, &msg );
    hasher.input_str(&text);
    let dup_check_key = hasher.result_str() ;
    // 检查是否有此消息了
    // let ttl_result:Result<usize, redis::RedisError> = con.ttl(&dup_check_key);  
    let ttl_result: usize  = con.ttl(dup_check_key.clone())?; 
    println!("ttl result :{:?}", ttl_result);
    println!("max usize :{:?}", usize::MAX);

    // 添加一条聊天记录后 紧跟着就可以缩减列表了
    let _ = con.rpush(key, &msg)?;
  
    // println!("{} => {}",text,hasher.result_str());
    let _ = con.set_ex(dup_check_key, 1, 30)? ;

    // ltrim 可以用来缩紧列表 不至于太长 适合消减聊天列表 ；服务器资源够的话 可以不必如此 或者用一个其他任务来转储部分旧的数据
    let _ = con.ltrim(key, -10, -1)?;

    let val: Vec<String> = con.lrange(key, -10, -1)?;
    println!("lset result: {:?}", val);

    Ok(())
}

fn usecase_sets(con: &mut redis::Connection) -> redis::RedisResult<()> {
    use redis::Commands; // 命令的高级方式（以方法名出现的那些命令）在此traits中实现

    let key = "my_set";

    let _ = con.sadd(key, vec![1, 3, 4])?;
    let success_count: usize = con.sadd(key, 5)?;
    println!("success insert count: {:?}", success_count);

    // 查看数量: `set Cardinality` 集合基数
    let scard: usize = con.scard(key)?;
    println!("scard: {:?}", scard);

    // 随机获取一条数据
    // 底层命令是有个可选参数count的 想弹出多条可以用循环来做
    let random_val: usize = con.spop(key)?;
    println!("random_val: {:?}", random_val);

    // 所有成员
    // NOTE： 慎用 大数据量时！
    let results: Vec<usize> = con.smembers(key)?;
    println!("results in set: {:?}", results);

    // 判断是否在集合
    let is_member: usize = con.sismember(key, 3)?;
    println!("is_member: {:?}", is_member);
    let is_member: usize = con.sismember(key, 8)?;
    println!("is_member: {:?}", is_member);

    // 删除元素
    let result: usize = con.srem(key, 4)?;
    let result: usize = con.srem(key, vec![1, 3])?;
    println!("delete result: {:?}", result);

    // 集合运算
    let _ = con.sadd("s_1", vec![1, 3, 4])?;
    let _ = con.sadd("s_2", vec![8, 9, 10, 3, 4])?;
    let sinter: Vec<usize> = con.sinter(vec!["s_1", "s_2"])?;
    println!("set intercet: {:?}", sinter);

    let sunion: Vec<usize> = con.sunion(vec!["s_1", "s_2"])?;
    println!("set sunion: {:?}", sunion);
    // 集合求差 顺序很重要 类似数学中的减数与被减数的关系
    let sdiff: Vec<usize> = con.sdiff(vec!["s_1", "s_2"])?;
    println!("set sdiff: {:?}", sdiff);

    // 工程应用
    // 去重 ； 集合运算 学生选课问题

    Ok(())
}
