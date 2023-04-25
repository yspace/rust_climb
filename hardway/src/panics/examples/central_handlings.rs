use std::any::Any;
use std::panic;
use std::thread;
use std::time;

fn main() {
    println!("Entering main!");

    let h = thread::spawn(|| {
        let dur_millis = time::Duration::from_millis(500);
        thread::sleep(dur_millis);
        panic!("boom");
    });

    let r = h.join();
    handle(r);

    let r = panic::catch_unwind(|| {
        let dur_millis = time::Duration::from_millis(500);
        thread::sleep(dur_millis);
        panic!(String::from("boom again!"));
    });

    handle(r);

    println!("Exiting main!");
}
//both of the panic::catch_unwind and thread::spawn  is to return Err(Any)
//fn handle(r: thread::Result<()>) {
fn handle<T: std::fmt::Debug>(r: Result<T, Box<dyn Any + Send + 'static>>) {
    //r: Box<T+ Send + 'static> is an owned pointer to a value (with the original type unknown and dynamically //change) such as std::any::Any , which can be sent across threads and lives as long as the program itself.
    //中文意思大概是： 这是一个指针， 指向动态大小的类型，所指value可以在线程间传递，并且其生命周期长度与本//进程一样长。
    println!("{:?}", r);
    match r {
        Ok(r) => println!("All is well! {:?}", r),
        Err(e) => {
            if let Some(e) = e.downcast_ref::<&'static str>() {
                println!("Got an error: {}", e);
            } else if let Some(e) = e.downcast_ref::<String>() {
                println!("Got an error: {}", e);
            } else {
                println!("Got an unknown error: {:?}", e);
            }
        }
    }
}
