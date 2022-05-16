pub fn run() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        println!(">> using tokio::runtime");

        // block_thread::run().await;
        // no_swapping::main().await;
        // await_to_sleep::main().await;
        rayon_spawn::main().await;
    })
}

mod block_thread {
    use std::time::Duration;

    pub async fn run() {
        println!("Hello World!");

        // No .await here!
        std::thread::sleep(Duration::from_secs(5));

        println!("Five seconds later...");
    }
}

mod no_swapping {
    use std::time::Duration;

    async fn sleep_then_print(timer: i32) {
        println!("Start timer {}.", timer);

        // No .await here!
        std::thread::sleep(Duration::from_secs(1));

        println!("Timer {} done.", timer);
    }

    pub async fn main() {
        // The join! macro lets you run multiple things concurrently.
        tokio::join!(
            sleep_then_print(1),
            sleep_then_print(2),
            sleep_then_print(3),
        );
    }
}

mod await_to_sleep {
    // await 是运行时的调度点 会做task切换
    use tokio::time::Duration;

    async fn sleep_then_print(timer: i32) {
        println!("Start timer {}.", timer);

        tokio::time::sleep(Duration::from_secs(1)).await;
        //                                            ^ execution can be paused here

        println!("Timer {} done.", timer);
    }

    // #[tokio::main]
    pub async fn main() {
        // The join! macro lets you run multiple things concurrently.
        // By using tokio::join!, all three tasks are guaranteed to run on the same thread
        tokio::join!(
            sleep_then_print(1),
            sleep_then_print(2),
            sleep_then_print(3),
        );
    }
}

mod spawn_blocking {
    // 线程池上限 在500左右
    pub async fn main() {
        // This is running on Tokio. We may not block here.

        let blocking_task = tokio::task::spawn_blocking(|| {
            // This is running on a thread where blocking is fine.
            println!("Inside spawn_blocking");
        });

        // We can wait for the blocking task like this:
        // If the blocking task panics, the unwrap below will propagate the
        // panic.
        blocking_task.await.unwrap();
    }
}

mod rayon_spawn {
    async fn parallel_sum(nums: Vec<i32>) -> i32 {
        let (send, recv) = tokio::sync::oneshot::channel();

        // Spawn a task on rayon.
        rayon::spawn(move || {
            // Perform an expensive computation.
            let mut sum = 0;
            for num in nums {
                sum += num;
            }

            // Send the result back to Tokio.
            let _ = send.send(sum);
        });

        // Wait for the rayon task.
        recv.await.expect("Panic in rayon::spawn")
    }

    pub async fn main() {
        /*
         The main danger of using rayon is that you must be careful not to block the thread while waiting for rayon to complete.
         To do this, combine rayon::spawn with tokio::sync::oneshot like this
        */

        let nums = vec![1; 1024 * 1024];
        // uses only one thread in the rayon thread pool per call to parallel_sum
        println!("{}", parallel_sum(nums.clone()).await);

        println!("{}",  parallel_sum2(nums).await );
    }

    
    async fn parallel_sum2(nums: Vec<i32>) -> i32 {
        // use rayon's parallel iterators to compute the sum on several threads:
        use rayon::prelude::*;

        let (send, recv) = tokio::sync::oneshot::channel();

        // Spawn a task on rayon.
        rayon::spawn(move || {
            // Compute the sum on multiple threads.
            let sum = nums.par_iter().sum();

            // Send the result back to Tokio.
            let _ = send.send(sum);
        });

        // Wait for the rayon task.
        recv.await.expect("Panic in rayon::spawn")
    }
}
