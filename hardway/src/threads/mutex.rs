// @see https://itsallaboutthebit.com/arc-mutex/
use std::time::Duration;
use std::{thread, thread::sleep};
use std::sync::{Arc, Mutex};
/*
## Mutex: add Sync to a Send type
Another interesting thing about Mutex is that as long as a type inside the Mutex is Send, Mutex will also be Sync. This is because Mutex ensures that only one thread can get access to the underlying value and thus it's safe to share Mutex between threads.


 */

// NOTE: 了解下 锁中毒 ；关于持有锁的线程出现错误终止时 所持锁是否被释放了？ https://doc.rust-lang.org/std/sync/struct.Mutex.html#poisoning

struct User {
    name: String
}

#[test]
fn main() {
    let user_original = Arc::new(Mutex::new(User { name: String::from("drogus") }));

    let user = user_original.clone();
    let t1 = thread::spawn(move || {
        let mut locked_user = user.lock().unwrap();
        locked_user.name = String::from("piotr");
        // after locked_user goes out of scope, mutex will be unlocked again,
        // but you can also explicitly unlock it with:
        // drop(locked_user);
    });

    let user = user_original.clone();
    let t2 = thread::spawn(move || {
        sleep(Duration::from_millis(10));

        // it will print: Hello piotr
        println!("Hello {}", user.lock().unwrap().name);
        /*
        In general leaving unrwap() methods in the production code is not recommended, 
        but in case of Mutex it might be a valid strategy - if a mutex has been poisoned we might decide that the application state is invalid and crash the application.
         */
    });

    t1.join().unwrap();
    t2.join().unwrap();
}

mod without_arc{

    //  this is not always possible, for example in async code, so Mutex is very often used along with Arc.
    
    use std::{sync::Mutex, thread::{scope, sleep}, time::Duration};

#[derive(Debug)]
struct User {
    name: String,
}
#[test]
fn main() {
    let user = Mutex::new(User {
        name: "drogus".to_string(),
    });

    scope(|s| {
        s.spawn(|| {
            user.lock().unwrap().name = String::from("piotr");
        });

        s.spawn(|| {
            sleep(Duration::from_millis(10));

            // should print: Hello piotr
            println!("Hello {}", user.lock().unwrap().name);
        });
    });
}
}