use sled::{Db, IVec};
use std::str;


// Rust：用Sled添加高性能嵌入式数据库
// @see https://juejin.cn/post/7345379893234712639
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 打开或创建一个新的Sled数据库
    let db: Db = sled::open("my_db")?;
    // 打开一个特定的Tree
    let tree = db.open_tree("my_tree")?;

    // 插入一些数据
    tree.insert("user1:John", "Doe")?;
    tree.insert("user2:Jane", "Doe")?;
    tree.insert("user3:Jake", "Smith")?;
    tree.insert("user4:Judy", "Brown")?;

    // 执行范围查询：查询以"user2"开头的键
    println!("Range query for keys starting with 'user2':");
    for item in tree.range("user2"..="user2\x7f") {
        let (key, value) = item?;
        println!("{}: {}", str::from_utf8(&key)?, str::from_utf8(&value)?);
    }

    // 执行前缀查询：查询所有以"user"前缀的键
    println!("\nPrefix query for keys starting with 'user':");
    for item in tree.scan_prefix("user") {
        let (key, value) = item?;
        println!("{}: {}", str::from_utf8(&key)?, str::from_utf8(&value)?);
    }


    println!("<< test 订阅功能");
    let _ = sub::main();
    println!("test 订阅功能 >>");

    Ok(())
}

mod sub{
    use sled::{Db, Event};

    pub fn main() -> Result<(), Box<dyn std::error::Error>> {
        // 打开或创建名为"my_db"的Sled数据库
        let db: Db = sled::open("my_db")?;

        // 订阅数据库中的所有前缀（即订阅所有变更事件）
        let mut events = db.watch_prefix("");

        // 在新线程中监听数据库变更事件
        std::thread::spawn(move || {
            // 遍历事件流
            for event in events {
                // 匹配不同类型的事件
                match event {
                    // 插入事件
                    Event::Insert { key, value } => {
                        // 当一个新的键值对被插入时，打印出键和值
                        println!("Inserted: {:?}, {:?}", key, value);
                    },
                    // 删除事件
                    Event::Remove { key } => {
                        // 当一个键值对被删除时，打印出键
                        println!("Removed: {:?}", key);
                    }
                }
            }
        });

        // 进行一些数据库操作以触发上面订阅的事件...

        // 插入一个键值对，触发插入事件
        db.insert("key4", "value4")?;
        // 删除刚刚插入的键值对，触发删除事件
        db.remove("key4")?;

        Ok(())
    }

    // 作者：Pomelo_刘金
    // 链接：https://juejin.cn/post/7345379893234712639
    // 来源：稀土掘金
    // 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
}