extern crate r2d2_redis;

use std::thread;

use r2d2_redis::{r2d2, redis::{self,Commands}, RedisConnectionManager};

// use redis::Commands;

fn main() {
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder()
        .build(manager)
        .unwrap();

    let mut handles = vec![];

    for _i in 0..10i32 {
        let pool = pool.clone();
        handles.push(thread::spawn(move || {
            let mut conn = pool.get().unwrap();
            let n: i64 = conn.incr("counter", 1).unwrap();
            println!("Counter increased to {}", n);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
}