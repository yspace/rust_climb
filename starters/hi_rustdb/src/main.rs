use rustdb::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建一个内存数据库
    // let mut db = Database::new(MemStorage::new());

    // // 创建一个表
    // db.execute("CREATE TABLE users (id INTEGER, name TEXT, age INTEGER)")?;
    //
    // // 插入数据
    // db.execute("INSERT INTO users VALUES (1, 'Alice', 30)")?;
    // db.execute("INSERT INTO users VALUES (2, 'Bob', 25)")?;
    // db.execute("INSERT INTO users VALUES (3, 'Charlie', 35)")?;
    //
    // // 查询数据
    // let mut result = db.execute("FOR id, name, age FROM users WHERE age > 25")?;
    //
    // // 处理查询结果
    // while let Some(row) = result.next()? {
    //     let id: i32 = row.get(0)?;
    //     let name: String = row.get(1)?;
    //     let age: i32 = row.get(2)?;
    //     println!("User: id={}, name={}, age={}", id, name, age);
    // }
    //
    // // 更新数据
    // db.execute("SET age = age + 1 FROM users WHERE name = 'Alice'")?;
    //
    // // 删除数据
    // db.execute("DELETE FROM users WHERE id = 2")?;

    Ok(())
}