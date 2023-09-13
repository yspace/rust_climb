https://ryhl.io/blog/async-what-is-blocking/

To give a sense of scale of how much time is too much, a good rule of thumb is no more than 10 to 100 microseconds between each .await. That said, this depends on the kind of application you are writing

### Read LIST

[Async Rust: What is a runtime? Here is how tokio works under the hood](https://kerkour.com/rust-async-await-what-is-a-runtime)
[Using Rustlang’s Async Tokio Runtime for CPU-Bound Tasks](https://thenewstack.io/using-rustlangs-async-tokio-runtime-for-cpu-bound-tasks/)

### 揭秘

```rust
#[async_std::main]
async fn main() {
}
```

https://github.com/async-rs/async-attributes/blob/master/src/lib.rs#L75

等价

```rust
let result = quote! {
        #vis fn main() #ret {
            #(#attrs)*
            async fn main(#inputs) #ret {
                #body
            }

            async_std::task::block_on(async {
                main().await
            })
        }

    };

    result.into()
```

书籍： https://book.async.rs/concepts/tasks.html

## 深入理解 async

https://github.com/nyxtom/async-in-depth-rust-series

### Futures

Futures 是在计算上的抽象

> Futures abstract over computation. They describe the "what", independent of the "where" and the "when". For that, they aim to break code into small, composable actions that can then be executed by a part of our system.

`Send` abstracts over passing data in a computation to another concurrent computation (let's call it the receiver), losing access to it on the sender side.

`Sync` is about sharing data between two concurrent parts of a program.

### An easy view of `computation`

While computation is a subject to write a whole book about, a very simplified view suffices for us: A sequence of composable operations which can branch based on a decision, run to succession and yield a result or yield an error

**Deferring computation**

As mentioned above, Send and Sync are about data. But programs are not only about data, they also talk about computing the data. And that's what Futures do.

> 线程用于并行，异步用于等待并行！

## tokio

_什么是 runtime_

https://kerkour.com/rust-async-await-what-is-a-runtime
黑帽子作者
Rust does not provide the execution context required to execute Futures and Streams. This execution context is called a runtime. You can't run an async Rust program without a runtime.

```rust
tokio::spawn(async move {
    for port in MOST_COMMON_PORTS_100 {
        let _ = input_tx.send(*port).await;
    }
});

```

This snippet of code spawns 1 task that will be pushed into the queue of one of the processors. As each processor have its own OS thread, by spawning a task, we use all the resources of our machine without having to manage threads ourselves. Without spawning, all the operations are executed on the same processor and thus the same thread.

As a matter of comparison, in Go we use the go keyword to spawn a task (called a goroutine):

```go
go doSomething()
```

NOTE The most important rule to remember in the world of async-await is not to block the event loop.
异步代码中 千万不要阻塞事件循环
超过 10-100 微秒 就要考虑 **spawn_blocking** 了

重 CPU 的任务
tokio provides the tokio::task::spawn_blocking function for blocking operations that eventually finish on their own. By that, I mean a blocking operation which is not an infinite background job. For this kind of task, a Rust Thread is more appropriate.

Instead, by calling spawn_blocking, the operation is dispatched to tokio's blocking tasks thread pool.

tokio 维护了两个线程池

一个定长的用于 executor 异步任务可以用**tokio::spawn**分派到此线程池上。
另一个动态长度 但有上限 最多 512 个线程 用于 blocking task ；此池根据阻塞任务的多少自动伸缩
tokio::task::spawn_blocking 可以发送阻塞任务到该池中 更详细信息见[in tokio's documentation](https://docs.rs/tokio/latest/tokio/runtime/struct.Builder.html#method.max_blocking_threads)

### 来自 influxdb 的博客

https://thenewstack.io/using-rustlangs-async-tokio-runtime-for-cpu-bound-tasks/

“If your code is CPU-bound and you wish to limit the number of threads used to run it, you should run it on another thread pool such as Rayon.”

I believe this wording caused significant confusion both within our team as well as in the broader Rust community. Many people read it to mean that a Tokio Runtime should never be used for CPU-bound tasks. The key point is actually that the same Runtime instance (the same thread pool) should not be used for both I/O and CPU, and we have subsequently clarified the intent of the docs (gory details on the PR).

Rayon 不支持 async！

Perhaps by thinking about a Tokio Runtime as a sophisticated thread pool, the idea of using different Runtime instances might seem more palatable, and we demonstrate how to do so with the dedicated executor below.

博文中提到了一个可用于 cpu intensive 的调度器 底层把任务发给 tokio

完整代码： https://github.com/influxdata/influxdb_iox/blob/fe155e15fb2ad166aee66b0458e63c24a8128dd4/query/src/exec/task.rs#L101-L118

```rust
pub struct DedicatedExecutor {
    state: Arc<Mutex<State>>,                                                                                                          
}                                                                                                                                      

/// Runs futures (and any `tasks` that are `tokio::task::spawned` by
/// them) on a separate Tokio Executor
struct State {                                                                                                    
    /// Channel for requests -- the dedicated executor takes requests                                                                  
    /// from here and runs them.
    requests: Option<std::sync::mpsc::Sender<Task>>,

    /// Thread which has a different Tokio runtime
    /// installed and spawns tasks there                                                                                            
    thread: Option<std::thread::JoinHandle<()>>,
}
 
impl DedicatedExecutor {
    /// Creates a new `DedicatedExecutor` with a dedicated Tokio
    /// executor that is separate from the threadpool created via                                                                      
    /// `[tokio::main]`.                                                                                                    
    pub fn new(thread_name: &str, num_threads: usize) -> Self {                                                                        
        let thread_name = thread_name.to_string();

        let (tx, rx) = std::sync::mpsc::channel::<Task>();

        let thread = std::thread::spawn(move || {
            // Create a new Runtime to run tasks                                                                                                                                                                                                                                
            let runtime = Tokio::runtime::Builder::new_multi_thread()                                                                  
                .enable_all()                                                                                                          
                .thread_name(&thread_name)
                .worker_threads(num_threads)
                // Lower OS priority of worker threads to prioritize main runtime                                                                                                                                                          
                .on_thread_start(move || set_current_thread_priority_low())
                .build()
                .expect("Creating Tokio runtime");

        // Pull task requests off the channel and send them to the executor
        runtime.block_on(async move {                                                                                                        
                while let Ok(task) = rx.recv() {                                                                                                                                                                                                                              
                    Tokio::task::spawn(async move {                                                                                    
                        task.run().await;                                                                                              
                    });                                                                                                                
                }                                                                
                                                                                                                                      
        let state = State {                                                                                                            
            requests: Some(tx),                                                                                                        
            thread: Some(thread),                                                                                                      
        };

        Self {
            state: Arc::new(Mutex::new(state)),                                                                                        
        }                                                                                                                              
    }
```

Note: The new thread is key. If you try to create a new Runtime on the main thread or one of the threads Tokio has created, you will get an error, as there is already a Runtime installed.

Here is the corresponding code to send a task to this second Runtime.

```rust


impl DedicatedExecutor {
 
    /// Runs the specified Future (and any tasks it spawns) on the
    /// `DedicatedExecutor`.                                                                        
    pub fn spawn<T>(&self, task: T) -> Job<T::Output>                                                                                  
    where                                                                                                                              
        T: Future + Send + 'static,                                                                                                    
        T::Output: Send + 'static,
    {                                                                                                                                  
        let (tx, rx) = tokio::sync::oneshot::channel();                                                                                

        let fut = Box::pin(async move {                                                                                                
            let task_output = task.await;                                                                                              
            tx.send(task_output).ok()                                                                                                                      
        });                                                                                                                            
        let mut state = self.state.lock();
        let task = Task {                                                                                                              
            fut,
        };

        if let Some(requests) = &mut state.requests {                                                                                  
            // would fail if someone has started shutdown                                                                              
            requests.send(task).ok();                                                                                                  
        } else {
            warn!("tried to schedule task on an executor that was shutdown");                                                          
        }                                                                                                                              

        Job { rx, cancel }
    }  
 } 
```

Job

The code above uses a wrapper around a Future called Job that handles transferring the results from the dedicated executor back to the main executor, which looks like:

~~~rust
/// Job within the executor.
///
/// Dropping the job will cancel its linked task.
#[pin_project(PinnedDrop)]
pub struct Job<T> {
    cancel: CancellationToken,
    #[pin]
    rx: Receiver<T>,
}

impl<T> Future for Job<T> {
    type Output = Result<T, Error>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();
        this.rx.poll(cx)
    }
}

#[pinned_drop]
impl<T> PinnedDrop for Job<T> {
    fn drop(self: Pin<&mut Self>) {
        self.cancel.cancel();
    }
}
~~~

### 一些启发

https://github.com/stevepryde/thirtyfour/blob/main/thirtyfour/examples/tokio_basic.rs

- [tokio 调度器](https://tokio.rs/blog/2019-10-scheduler)

工作窃取

- [深入了解 Rust 异步开发模式](https://xie.infoq.cn/article/0b2cf4ce21a9ff65a833e7116)

- [Async Rust Is A Bad Language](https://bitbashing.io/async-rust.html)
https://www.kernel.org/doc/html/next/RCU/whatisRCU.html

https://journal.stuffwithstuff.com/2015/02/01/what-color-is-your-function/
