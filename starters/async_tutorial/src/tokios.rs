use std::time::Duration;

use tokio::time::sleep;


pub   fn run(){

        tokio::runtime::Runtime::new().unwrap().block_on(async {
            // println!("Hello world");
            // _main().await;
            tasks::main().await;
        })
}



async fn _main() {
    // use tokio::join! to run multiple futures concurrently.
    let (v1, v2, v3) = tokio::join!(
        async {
            sleep(Duration::from_millis(1500)).await;
            println!("Value 1 ready");
            "Value 1"
        },
        async {
            // 标准库中的sleep blocks an entire thread
            // std::thread::sleep(Duration::from_millis(2800));
            // sleep(Duration::from_millis(2800)).await;

            /*
            The tokio::task module contains an implementation of green threads,
            similar to Go’s goroutines. With spawn_blocking, you can get the Tokio runtime to run blocking code inside a dedicated thread pool,
            allowing other futures to continue making progress.
             */
            tokio::task::spawn_blocking(|| {
                std::thread::sleep(Duration::from_millis(2800));
            })
            .await
            .unwrap();

            println!("Value 2 ready");
            "Value 2"
        },
        async {
            sleep(Duration::from_millis(600)).await;
            println!("Value 3 ready");
            "Value 3"
        },
    );

    assert_eq!(v1, "Value 1");
    assert_eq!(v2, "Value 2");
    assert_eq!(v3, "Value 3");
}


mod tasks {
    // tokio::task::spawn, an async version of std::thread::spawn.

    #[derive(Debug)]
    enum Message {
        SendWelcomeEmail { to: String },
        DownloadVideo { id: usize },
        GenerateReport,
        Terminate,
    }

    use std::sync::Arc;
    use std::time::Duration;
    // tokio::sync::mpsc::channel, a limited-size channel that provides back pressure when your application is under load to prevent it from being overwhelmed.
    use tokio::sync::mpsc::unbounded_channel;
    use tokio::sync::Mutex;
    use tokio::time::sleep;

    // #[tokio::main]
    pub async fn main() {
        //  async alternative to the MPSC channel in the standard library
        let (sender, receiver) = unbounded_channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let size = 5;
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let receiver = Arc::clone(&receiver);
            let worker = tokio::spawn(async move {
                loop {
                    let message = receiver
                        // 这里有个Guard 保证只有一个task能获得锁
                        .lock()
                        .await
                        .recv()
                        .await
                        .unwrap_or_else(|| Message::Terminate);

                    println!("Worker {}: {:?}", id, message);
                    
                    match message {
                        Message::Terminate => break,
                        // match against each message you want to handle and put some real logic here.
                        _ => sleep(Duration::from_secs(1 + id as u64)).await,
                    }
                }
            });
            workers.push(worker);
        }

        for _ in &workers {
            /*
            Note that we didn’t use await after the send method here.
            That’s because we used an unbounded channel, which will never block a sender. Bounded channels can block the sender so you would need to use await when sending.
            You could also use try_send.
             */
            // let _ = sender.send(Message::Terminate);

            sender.send(Message::DownloadVideo { id: 10 }).unwrap();
            sender.send(Message::GenerateReport).unwrap();
            sender
                .send(Message::SendWelcomeEmail {
                    to: "hi@example.com".into(),
                })
                .unwrap();
            sender.send(Message::DownloadVideo { id: 25 }).unwrap();
        }
        for worker in workers {
            let _ = worker.await;
        }
    }
}
