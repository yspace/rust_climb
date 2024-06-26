use std::{io, thread, time::SystemTime};

use super::*;

// use redis::Commands;
use r2d2_redis::redis::Commands;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    text: String,
    time: String,
}

pub fn run(context: &mut AppContext) -> redis::RedisResult<()> {
    // usecase_hash(&mut con);

    let pool = context.redis_pool.clone();
    let mut conn = pool.get().unwrap();

     //
     println!("多线程里面启动一个接收者");
     // todo: 改造为多线程运行
     let pool2 = pool.clone();
     let mut _handle = thread::spawn(move || {
         receiver::run(pool2).unwrap();
     });
     // 需不需要join呢！😄

    loop {
        let mut s = String::new();
        io::stdin()
            .read_line(&mut s) // 输入一行字符串
            .expect("failed to read line.");
        println!("{}", s); // 输出字符串

        // 当前时间
        // let t = std::time::SystemTime::now();
        // t.duration_since(SystemTime::UNIX_EPOCH);
        let current_time = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        // println!("current time: {}", current_time);
        let msg = Message {
            text: s,
            time: current_time,
        };
        // println!("{:?}", msg);
        let json: String = serde_json::to_string(&msg).unwrap();
        println!("{}", json);
        let _: () = conn.set("message", json).unwrap();
    }

   

    Ok(())
}

mod sender {
    use super::*;

    pub fn run(pool: Pool<RedisConnectionManager>) -> redis::RedisResult<()> {
        let mut conn = pool.get().unwrap();

        let key = "message";
        let json_str: String = conn.get(key).unwrap();
        let msg: Message = serde_json::from_str(&json_str).unwrap();
        println!("{:#?}", msg);

        Ok(())
    }
}

mod receiver {
    use std::time::Duration;

    use super::*;

    pub fn run(pool: Pool<RedisConnectionManager>) -> redis::RedisResult<()> {
        let mut conn = pool.get().unwrap();

        println!("begin receive msg");
        // let last_msg_time :Option<String> = None;
        let mut last_msg_time  = "".to_string();
        loop {
            let key = "message";
            let json_str: Option<String> = conn.get(key).unwrap();
            if json_str.is_some() {

                let msg: Message = serde_json::from_str(&json_str.unwrap()).unwrap();
    
                if last_msg_time == msg.time {
                    thread::sleep(Duration::from_secs(1));
                    continue; 
                }else{

                    println!("{:#?}", msg);
                    last_msg_time = msg.time;
                }
            }else{
                thread::sleep(Duration::from_secs(1));
                continue; // 这行在这里 基本可以去掉
            }
        }

        Ok(())
    }
    pub fn run_with_Context(context: &mut AppContext) -> redis::RedisResult<()> {
        let pool = context.redis_pool.clone();
        let mut conn = pool.get().unwrap();

        let key = "message";
        let json_str: String = conn.get(key).unwrap();
        let msg: Message = serde_json::from_str(&json_str).unwrap();
        println!("{:#?}", msg);

        Ok(())
    }
}
