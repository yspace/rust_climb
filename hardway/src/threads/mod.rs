use std::{
    sync::{Arc, RwLock},
    thread::spawn,
};

mod copy_types;

pub fn main() {
    _threads::main();
    _threads::usecase_closure();
    _threads::usecase_value_from_thread();
    _threads::builder();
    return;
    // ==

    basic();
    learn_move();

    copy_types::main();
}

fn basic() {
    let list = vec!["hello", "world"];

    let mut handlers = vec![];
    for i in 0..10 {
        // spawn is short hand for thread::Builder::new().spawn().unwrap()
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

mod _threads {
    use std::{thread, iter::FromIterator};
    pub fn main() {
        let t1 = thread::spawn(f);
        let t2 = thread::spawn(f);
        println!("msg from the main thread.");

        // 等
        /**
         * The .join() method will wait until the thread has finished executing, and returns a thread::Result.
         * If the thread did not successfully finish its function because it panicked, this will contain the panic message.
         */
        t1.join().expect("Could not join thread");
        t2.join().unwrap();

        //
        let t_panic = thread::spawn(f_panic);
        match t_panic.join() {
            Ok(rslt) => {}
            Err(err) => {
                println!("err from panic thread: {:?}", err);
            }
        }
    }

    fn f() {
        // The println macro uses io::Stdout::lock() to make sure its output does not get interrupted.
        println!("from child thread!");
        let id = thread::current().id();
        println!("thread id is: {:?}", id);
    }

    fn f_panic() {
        println!("from child thread panic!");

        panic!("hi this thread is paniced!");
    }

    pub fn usecase_closure() {
        let numbers = vec![1, 2, 3];
        // 捕获外部作用域中的变量
        thread::spawn(move || {
            for n in numbers {
                println!("{n}");
            }
        })
        .join()
        .unwrap();
    }

    pub fn usecase_value_from_thread() {
        let numbers = Vec::from_iter(0..=100);
        let t = thread::spawn(move || {
            let len = numbers.len();
            let sum = numbers.into_iter().sum::<usize>();
            sum / len
        });
        let average = t.join().unwrap();
        println!("average: {average}");
    }

    pub fn builder(){
        // 此builder具有更多的可配置性 可防御性 thread::spawn方法是一把梭的 
        let mut tb = thread::Builder::new();
        tb = tb.name("my_thread".to_string());
        // tb.stack_size(1024) ;
        //  spawning a new thread fails. 
        // This might happen if the operating system runs out of memory, or if resource limits have been applied to your program. 
        let rslt = tb.spawn(||{
            println!("thread name: {}", thread::current().name().unwrap()) ;
        });

        rslt.unwrap().join().unwrap();

    }
}

