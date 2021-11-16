use std::{
    sync::{Arc, RwLock},
    thread::spawn,
};

mod copy_types;

pub fn main() {
    basic();
    learn_move();

    copy_types::main();
}

fn basic() {
    let list = vec!["hello", "world"];

    let mut handlers = vec![];
    for i in 0..10 {
        let jh = spawn(move || {
            println!("hello from thread: #{}", i);
        });
        handlers.push(jh);
    }

    for handle in handlers {
        handle.join().expect("Could not join thread");
    }

    println!("Result: {}", list.join(","));
}

fn learn_move() {
    let list = vec!["hello".to_string(), "world".to_string()];

    // 创建共享的安全内存结构
    //    let mut rwlock = RwLock::new(&list) ;
    let mut rwlock = RwLock::new(list);
    let mut arc = Arc::new(rwlock);

    let mut handlers = vec![];
    for i in 0..10 {
        let arc_clone = arc.clone();
        let jh = spawn(move || {
            let mut list = arc_clone.write().expect("Could not get write lock!");

            list.push(format!("Thread #{}", i));
            // println!("hello from thread: #{}", i) ;
        });
        handlers.push(jh);
    }

    for handle in handlers {
        handle.join().expect("Could not join thread");
    }

    let list = arc.read().expect("Could not get read loack");

    println!("Result: {}", list.join(","));
}
