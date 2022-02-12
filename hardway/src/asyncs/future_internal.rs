/**
 * futrue 在其他语言中可能对应的是Promise 承诺的意思
 * 比如 借钱时的承诺 此后需要你拿着借条去兑现（主动去要） 每次去借钱时的孙子可能就变大爷了
 * 场景：
 *  大爷  有钱还没钱还呀
 *  对方答复 会出现两种情况：
 *      1. 有 给你准备好了Ready(100$)
 *      2. 没有 你过段时间再来吧
 * 你每次去要帐(<<poll>> 操作) 都是这两种结果
 * 
 * 对于没有的情况 如果不提供wake方法 你就必须去轮询了
 * 
 * 当然借条可以联合成更大借条 大借条里面套小借条 
 */
mod for_sample{
    trait SimpleFuture {
        type Output ;
        fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
    }
    
    enum Poll <T>{
        Ready(T),
        Pending ,
    }
    
    /*
     * @see https://course.rs/async/future-excuting.html
     * ======
     * 
    pub struct SocketRead<'a> {
        socket: &'a Socket,
    }
    
    impl SimpleFuture for SocketRead<'_> {
        type Output = Vec<u8>;
    
        fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
            if self.socket.has_data_to_read() {
                // socket有数据，写入buffer中并返回
                Poll::Ready(self.socket.read_buf())
            } else {
                // socket中还没数据
                // 
                // 注册一个`wake`函数，当数据可用时，该函数会被调用，
                // 然后当前Future的执行器会再次调用`poll`方法，此时就可以读取到数据
                self.socket.set_readable_callback(wake);
                Poll::Pending
            }
        }
    }
    =======
    这种 Future 模型允许将多个异步操作组合在一起，同时还无需任何内存分配。不仅仅如此，如果你需要同时运行多个 Future或链式调用多个 Future ，
    也可以通过无内存分配的状态机实现，例如：
    
    /// 一个SimpleFuture，它会并发地运行两个Future直到它们完成
    ///
    /// 之所以可以并发，是因为两个Future的轮询可以交替进行，一个阻塞，另一个就可以立刻执行，反之亦然
    pub struct Join<FutureA, FutureB> {
        // 结构体的每个字段都包含一个Future，可以运行直到完成.
        // 如果Future完成后，字段会被设置为 `None`. 这样Future完成后，就不会再被轮询
        a: Option<FutureA>,
        b: Option<FutureB>,
    }
    
    impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
    where
        FutureA: SimpleFuture<Output = ()>,
        FutureB: SimpleFuture<Output = ()>,
    {
        type Output = ();
        fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
            // 尝试去完成一个 Future `a`
            if let Some(a) = &mut self.a {
                if let Poll::Ready(()) = a.poll(wake) {
                    self.a.take();
                }
            }
    
            // 尝试去完成一个 Future `b`
            if let Some(b) = &mut self.b {
                if let Poll::Ready(()) = b.poll(wake) {
                    self.b.take();
                }
            }
    
            if self.a.is_none() && self.b.is_none() {
                // 两个 Future都已完成 - 我们可以成功地返回了
                Poll::Ready(())
            } else {
                // 至少还有一个 Future 没有完成任务，因此返回 `Poll::Pending`.
                // 当该 Future 再次准备好时，通过调用`wake()`函数来继续执行
                Poll::Pending
            }
        }
    }
     */
}


use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

 pub struct TimerFuture {
     shared_state: Arc<Mutex<SharedState>> ,
 }


/// 在Future和等待的线程间共享状态
struct SharedState {
    /// 定时(睡眠)是否结束
    completed: bool,

    /// 当睡眠结束后，线程可以用`waker`通知`TimerFuture`来唤醒任务
    waker: Option<Waker>,
}

impl Future for TimerFuture{
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> std::task::Poll<Self::Output> {
       // 通过检查共享状态，来确定定时器是否已经完成
       let mut shared_state = self.shared_state.lock().unwrap();
       if shared_state.completed {
           Poll::Ready(())
       } else {
           // 设置`waker`，这样新线程在睡眠(计时)结束后可以唤醒当前的任务，接着再次对`Future`进行`poll`操作, 
           // 
           // 下面的`clone`每次被`poll`时都会发生一次，实际上，应该是只`clone`一次更加合理。
           // 选择每次都`clone`的原因是： `TimerFuture`可以在执行器的不同任务间移动，如果只克隆一次，
           // 那么获取到的`waker`可能已经被篡改并指向了其它任务，最终导致执行器运行了错误的任务
           shared_state.waker = Some(cx.waker().clone());
           Poll::Pending
       }
    }

    impl TimerFuture {
        /// 创建一个新的`TimerFuture`，在指定的时间结束后，该`Future`可以完成
        pub fn new(duration: Duration) -> Self {
            let shared_state = Arc::new(Mutex::new(SharedState {
                completed: false,
                waker: None,
            }));
    
            // 创建新线程
            let thread_shared_state = shared_state.clone();
            thread::spawn(move || {
                // 睡眠指定时间实现计时功能
                thread::sleep(duration);
                let mut shared_state = thread_shared_state.lock().unwrap();
                // 通知执行器定时器已经完成，可以继续`poll`对应的`Future`了
                shared_state.completed = true;
                if let Some(waker) = shared_state.waker.take() {
                    waker.wake()
                }
            });
    
            TimerFuture { shared_state }
        }
    }

    // 上例中的XxxFuture 自己实现的话 其思想主要就是 什么时候可以还钱了
    // 对于计时器Future 就是超时状态到了 poll方法就是催债来了 你只要不决定还钱
    // 那么这个催债动作还是会一直在后面的时间里进行下去的
}