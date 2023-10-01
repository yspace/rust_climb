// The border between things is where the most interesting events take place.
// — David Holmgren on permaculture
// @see https://thomask.sdf.org/blog/2021/03/08/bridging-sync-async-code-in-rust.html

use crossbeam::channel;
use std::error::Error;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::runtime::Handle;

async fn get_score_async() -> Result<u32, Box<dyn Error + Send + Sync>> {
    let mut conn = TcpStream::connect("172.17.66.179:4444").await?;
    let mut score_str = String::new();
    let _ = conn.read_to_string(&mut score_str).await?;
    Ok(score_str.parse()?)
}

fn get_score_sync(handle: Handle) -> Result<u32, Box<dyn Error + Send + Sync>> {
    let (tx, rx) = channel::bounded(1);
    handle.spawn(async move {
        let score_res = get_score_async().await;
        let _ = tx.send(score_res);
    });
    Ok(rx.recv()??)
}

mod blocking_call_from_async_code {

    use std::thread;
    use std::time::Duration;

    fn long_running_task() -> u32 {
        thread::sleep(Duration::from_secs(5));
        5
    }

    async fn my_task() {
        let res = tokio::task::spawn_blocking(|| long_running_task())
            .await
            .unwrap();
        println!("The answer was: {}", res);
    }

    #[test]
    fn it_works() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let res = rt.block_on(async { my_task().await });
    }
}

mod block_on {
    use futures::executor::block_on;

    async fn hello() -> String {
        return String::from("Hello world!");
    }

    #[test]
    fn main() {
        let output = block_on(hello());
        println!("{output}");
    }
}
