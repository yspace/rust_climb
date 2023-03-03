fn main() {
    println!("Hello, world!");

    let result = do_something();
    match result {
        Ok(_) => println!("ok!"),
        Err(err) => println!("error: {:?}", err),
    }

}

fn do_something() -> redis::RedisResult<()> {
    use redis::Commands;
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    let _ : () = con.set("key", "hello")?;
    let _ : () = con.set("my_key", 42)?;
    let _ : () = con.set("my_key2", 43)?;
    let _ : () = con.set_nx("my_key", 41)?;
    let _ : () = con.append("key", " redis")?;


    let k : Option<String> = con.get("key")?;
    /* do something here */
    println!("{:?}", k.unwrap());

    // let keys :  Vec<String> = con.keys("*").unwrap();
    // println!("{:?}", keys);

    println!("{:?}",do_something2(&mut con).unwrap());
    println!("{:?}",do_something3(&mut con).unwrap());

    usecase_keys(&mut con)?;

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

    let key = "counter" ;
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

    // However, this will work as you'd expect
    // redis::cmd("MEMORY").arg("USAGE").arg("my_key").query(con)
    Ok(())
}
