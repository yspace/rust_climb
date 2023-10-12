https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton

```rust

#![feature(once_cell)] // 1.67.0-nightly
use std::sync::{LazyLock, Mutex};

static ARRAY: LazyLock<Mutex<Vec<u8>>> = LazyLock::new(|| Mutex::new(vec![]));

fn do_a_call() {
    ARRAY.lock().unwrap().push(1);
}

fn main() {
    do_a_call();
    do_a_call();
    do_a_call();

    println!("called {}", ARRAY.lock().unwrap().len());
}


```


// @see https://github.com/paulkernfeld/global-data-in-rust

https://www.sitepoint.com/rust-global-variables/

### once_cell

 ~~~rust

    use once_cell::sync::OnceCell;
    static POOL : OnceCell<Pool<MySql>>  = OnceCell::new();

    #[tokio::main]
    async fn main() -> Result<()>{
        POOL.set(MySqlPoolOptions::new().some_setting(some_value)
        .connect(mysql_connection_str).await?).unwrap();
    }
 ~~~