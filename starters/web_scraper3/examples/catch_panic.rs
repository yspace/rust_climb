
// 有空了 👀👀这个：https://crates.io/crates/shellflip

use tokio::task;
use std::io;
use std::panic;

#[tokio::main]
async fn main() -> io::Result<()> {
    // let join_handle: task::JoinHandle<Result<i32, io::Error>> = tokio::spawn(async {
    //     panic!("boom");
    // });
    //
    // let err = join_handle.await.unwrap_err();
    // assert!(err.is_panic());

    recipe2::main().await ;

    Ok(())
}

mod recipes{
    use std::time::Duration;

    use futures::FutureExt;
    async fn main_1() {
        loop {
            let task = tokio::spawn(async move {
                let mut cnt = 1;
                loop {
                    if cnt >= 5 {
                        panic!("ooo, paniced");
                    } else {
                        println!("count is {cnt} now");
                        cnt += 1;
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                }
            });

            let unwind = task.catch_unwind().await;

            if let Ok(Err(err)) = unwind {
                println!("unwind panic happen {err:?}, loop will restart task");
            }
        }
    }
}

mod recipe2{
    use tokio::task::JoinHandle;
    use tokio::time::{sleep, Duration};
    use std::error::Error;
    use anyhow::{Context, Result};

    // 示例任务函数，可能失败
    async fn risky_task(id: u64) -> Result<()> {
        println!("任务 {} 开始执行", id);
        // 模拟可能失败的操作
        if id % 2 == 0 {
            return Err(anyhow::anyhow!("模拟错误: 任务 {} 失败", id));
        }
        sleep(Duration::from_millis(100)).await;
        println!("任务 {} 成功完成", id);
        Ok(())
    }

    // 重启逻辑：捕获错误并重试
    async fn restartable_task(id: u64, max_retries: u32) -> Result<()> {
        let mut attempt = 0;
        loop {
            let handle = tokio::spawn(async move {
                risky_task(id).await
            });

            // 等待任务完成并捕获结果
            match handle.await {
                Ok(Ok(())) => {
                    println!("任务 {} 成功，无需重启", id);
                    return Ok(());
                }
                Ok(Err(e)) => {
                    attempt += 1;
                    if attempt >= max_retries {
                        eprintln!("任务 {} 失败，已达到最大重试次数: {}", id, e);
                        return Err(e);
                    }
                    let delay = Duration::from_millis(100 * 2_u64.pow(attempt - 1)); // 指数退避
                    println!("任务 {} 失败 (尝试 {}/{}): {}. 等待 {:?} 后重试...",
                             id, attempt, max_retries, e, delay);
                    sleep(delay).await;
                }
                Err(join_err) => {
                    attempt += 1;
                    if attempt >= max_retries {
                        eprintln!("任务 {} panic: {}", id, join_err);
                        return Err(anyhow::anyhow!("任务崩溃: {}", join_err));
                    }
                    let delay = Duration::from_millis(100 * 2_u64.pow(attempt - 1));
                    println!("任务 {} panic (尝试 {}/{}): {}. 等待 {:?} 后重试...",
                             id, attempt, max_retries, join_err, delay);
                    sleep(delay).await;
                }
            }
        }
    }

    // 主函数示例
    // #[tokio::main]
   pub async fn main() -> Result<()> {
        // 启动多个重启任务
        let mut handles = vec![];
        for i in 1..=3 {
            let handle = tokio::spawn(async move {
                restartable_task(i, 3).await
            });
            handles.push(handle);
        }

        // 等待所有任务完成
        for handle in handles {
            handle.await?;
        }
        Ok(())
    }

}