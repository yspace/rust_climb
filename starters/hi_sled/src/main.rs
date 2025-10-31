use sled::{Db, IVec};
use std::str;

mod error;

// Rust：用Sled添加高性能嵌入式数据库
// @see https://juejin.cn/post/7345379893234712639
// @see https://github.com/sgchris/sledoview 这个工具能👀看sled结构
// 不错的使用案例： https://bpfman.io/v0.5.6/blog/2024/01/15/bpfmans-shift-towards-a-daemonless-design-and-using-sled-a-high-performance-embedded-database/
// 这个应用 对sled 有很深的应用 https://github.com/bpfman/bpfman/blob/main/bpfman/src/types.rs

// @see https://github.com/atomicdata-dev/atomic-server/blob/master/lib/src/db.rs
// @see https://docs.rs/sled-extensions/latest/sled_extensions/
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 打开或创建一个新的Sled数据库
    let db: Db = sled::open("my_db")?;
    // 打开一个特定的Tree
    // 💪关于作为树的相关方法 参考： https://docs.rs/sled/latest/sled/struct.Tree.html
    let tree = db.open_tree("my_tree")?;

    // 插入一些数据
    tree.insert("user1:John", "Doe")?;
    tree.insert("user2:Jane", "Doe")?;
    tree.insert("user3:Jake", "Smith")?;
    tree.insert("user4:Judy", "Brown")?;

    /**

    在这个例子中，我们首先插入了一些以"user"开头，后面跟随用户名和姓氏的键值对。之后，我们展示了如何进行范围查询和前缀查询：

    范围查询：通过使用tree.range("user2"..="user2\xff")，我们查询了所有键在"user2"到"user2\xff"（一个高于"user2"的任何可能值的字符串）范围内的键值对。这在实际应用中可以用于查找特定范围内的记录。
    前缀查询：通过tree.scan_prefix("user")，我们查询了所有以"user"为前缀的键值对。这对于获取具有共同前缀的所有记录非常有用，例如，按用户名或分类检索数据。

    作者：Pomelo_刘金
    链接：https://juejin.cn/post/7345379893234712639
    来源：稀土掘金
    著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。

    */
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

    println!("<< test 基础功能");
    let _ = basic::main();
    println!("test 基础功能>>");

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

mod basic{
    // @see https://cloud.tencent.com/developer/article/1638678
    use sled::Db;
    pub fn main() -> Result<(), Box<dyn std::error::Error>> {
        // 打开数据库
        let tree = sled::open("/tmp/welcome-to-sled").expect("open");

        // 插入KV，读取Key对应的值
        tree.insert("KEY1", "VAL1");
        tree.insert("KEY2", "VAL2");
        tree.insert("KEY3", "VAL3");
        assert_eq!(tree.get(&"KEY1"), Ok(Some(sled::IVec::from("VAL1"))));

        // 范围查询
        for kv in tree.range("KEY1".."KEY9") {
           // ...
            let (key, value) = kv?;
            println!("{}: {}", str::from_utf8(&key)?, str::from_utf8(&value)?);
        }


        // 删除
        tree.remove(&"KEY1");

        // atomic compare and swap，可以用在并发编程中
        tree.compare_and_swap("KEY1", Some("VAL1"), Some("VAL2"));

        // 阻塞直到所有修改都写入硬盘
        tree.flush();

        Ok(())
    }
}