use std::sync::{Arc,Mutex};
use std::thread;


fn main() {
    let s = String::from("hello");

    let mutex = Mutex::new(s.to_owned());

    let arc = Arc::new(mutex);


    let arc2 = arc.clone();
    let handle = thread::spawn(move||{
        // lock 之所以不需要手动显式释放 是因为使用了一个rust设计惯用法`XxxGuard`
        arc2.lock().unwrap().push_str(" thread ");
    });
     
    {
        arc.lock().unwrap().push_str(" rust ");
    }

    handle.join().unwrap();
    println!("{:?}",arc);
}
