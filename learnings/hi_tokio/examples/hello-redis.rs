// use tokio::sync::mpsc;

async fn say_to_world() -> String {
    String::from("world")
}

#[tokio::main]
async fn main() {
  client::main().await.expect("client::main() failed");
  println!("> exit main!");
}

async fn run() {
    // 此处的函数调用是惰性的，并不会执行 `say_to_world()` 函数体中的代码
    let op = say_to_world();

    // 首先打印出 "hello"
    println!("hello");

    // 使用 `.await` 让 `say_to_world` 开始运行起来
    println!("{}", op.await);

    //
    my_channel().await;
}

mod client {
    use mini_redis::{client, Result};

    pub async fn main() -> Result<()> {
        // Open a connection to the mini-redis address.
        let mut client = client::connect("127.0.0.1:6379").await?;

        // Set the key "hello" with value "world"
        client.set("hello", "world".into()).await?;

        // Get key "hello"
        let result = client.get("hello").await?;

        println!("got value from the server; result={:?}", result);

        Ok(())
    }
}

async fn my_channel() {
    use futures::{channel::mpsc, SinkExt, StreamExt};
    const BUFFER_SIZE: usize = 2;
    let (mut tx, mut rx) = mpsc::channel::<i32>(BUFFER_SIZE);

    let mut tx2 = tx.clone();

    tx.send(1).await.unwrap();
    // 关闭tx
    drop(tx);

    tx2.send(2).await.unwrap();
    drop(tx2);

    println!("{:?}", rx.next().await);
    println!("{:?}", rx.next().await);
    println!("{:?}", rx.next().await);
    println!("{:?}", rx.next().await);
    println!("{:?}", rx.next().await);
}
