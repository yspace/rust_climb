
redis 

管道操作
~~~rust
let ((k1, k2),) : ((i32, i32),) = redis::pipe()
    .cmd("SET").arg("key_1").arg(42).ignore()
    .cmd("SET").arg("key_2").arg(43).ignore()
    .cmd("MGET").arg(&["key_1", "key_2"]).query(&con).unwrap();
~~~


## 参考
- [24 days of Rust articles](https://siciarz.net/24-days-of-rust-redis/)
共同朋友这个不错
~~~rust
extern crate redis;

use redis::{Client, Commands, Connection, RedisResult};
use std::collections::HashSet;

type UserId = u64;

fn add_friend(conn: &Connection, my_id: UserId, their_id: UserId) -> RedisResult<()> {
    let my_key = format!("friends:{}", my_id);
    let their_key = format!("friends:{}", their_id);
    let _: () = try!(conn.sadd(my_key, their_id));
    let _: () = try!(conn.sadd(their_key, my_id));
    Ok(())
}

fn friends_in_common(conn: &Connection, my_id: UserId, their_id: UserId) -> RedisResult<HashSet<UserId>> {
    let my_key = format!("friends:{}", my_id);
    let their_key = format!("friends:{}", their_id);
    Ok(try!(conn.sinter((my_key, their_key))))
}

~~~


- https://github.com/kkharji/redis-derive




- [Getting Started with Rust and Vultr Managed Databases for Redis](https://www.vultr.com/docs/getting-started-with-rust-and-vultr-redis-managed-database/)
两种命令风格 高级别DSL式的（比较人性 但命令支持不完全） 和低级别命令（低级别是可以完成所有redis支持的功能）

官方库有多个特性开关：
* 集群
* geospatial
* tls