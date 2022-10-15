use std::future::Future;
use std::sync::Arc;
use std::task::{Context, Poll, Wake};
use std::thread::{self,Thread} ;

// 当调用时唤醒当前线程
struct ThreadWaker(Thread) ;

impl Wake for ThreadWaker{
    fn wake(self: Arc<Self>) {
        self.0.unpark() ;
    }
}

fn block_on<T>(fut: impl Future<Output=T>) -> T{
    let mut fut = Box::pin(fut);

    let t = thread::current() ;
    let waker = Arc::new(ThreadWaker(t)).into();
    let mut ctx = Context::from_waker(&waker);

    loop{
        match fut.as_mut().poll(&mut ctx){
            Poll::Ready(res) => return res ,
            Poll::Pending => thread::park(),
        }
    }
}

async fn async_foo()-> usize{
    666
}

pub fn main(){
    block_on(async{
        println!(" Hello from inside a future");
    });

    println!("test:{}", block_on(async_foo()));
}