use std::{
    sync::{Arc, mpsc::channel, Mutex},
    thread::{self, sleep}, cell::RefCell, borrow::Borrow,
};

// use async_std::sync::Mutex;

#[test]
fn test_1() {
    println!("hello threading");

    let steps = 5;

    let thread_handle = thread::spawn(move || {
        for step in 1..steps {
            sleep(std::time::Duration::from_secs(1));

            println!("Thread step {}", step);
        }

        // println!("Goodbye!");
        "GoodBye!"
    });
    println!("spawn a thread !");
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("slept 2 secs ,Now joining thread...");
    let result = thread_handle.join().unwrap();

    println!("thread returned with {}", result);
}

#[test]
fn test_2() {
    println!("hello threading");

    let steps = Arc::new(5);

    let thread_handle = {
        let steps = steps.clone();
        thread::spawn( move|| {
            for step in 1..*steps {
                sleep(std::time::Duration::from_secs(1));

                println!("Thread step {}", step);
            }

            // println!("Goodbye!");
            "GoodBye!"
        })
    };
    println!("spawn a thread ! with step count of {}", steps);
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("slept 2 secs ,Now joining thread...");
    let result = thread_handle.join().unwrap();

    println!("thread returned with {}", result);
}


#[test]
fn test_mpsc() {
    let (sender, receiver) = channel();

    let thread_handle = {
        thread::spawn( move|| {
            let steps = receiver.recv().unwrap();
            for step in 1..steps {
                sleep(std::time::Duration::from_secs(1));

                println!("Thread step {}", step);
            }

            // println!("Goodbye!");
            "GoodBye!"
        })
    };

    println!("spawn a thread !  " );
    let _ = sender.send(5); // 接收端如果关闭了 send的结果也可能是error的！
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("slept 2 secs ,Now joining thread...");
    let result = thread_handle.join().unwrap();

    println!("thread returned with {}", result);
}




/*
// 有问题
#[test]
fn test_() {
    println!("hello threading");

    // 试着把 RefCell 换成Mutex
    let steps = Arc::new(RefCell::new(5));

    let thread_handle = {
        let steps = steps.clone();
        thread::spawn( move|| {
            while *steps.borrow() > 0{

                sleep(std::time::Duration::from_secs(1));

                println!("Thread step {}", steps.borrow());

                *steps.borrow_mut() -= 1;
            }

            // println!("Goodbye!");
            "GoodBye!"
        })
    };
    println!("spawn a thread ! with step count of {}", steps.borrow());
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("slept 2 secs ,Now joining thread...");
    let result = thread_handle.join().unwrap();

    println!("thread returned with {}", result);
}
 */

#[test]
fn test_mutex() {
    println!("hello threading");

    // 试着把 RefCell 换成Mutex
    let steps = Arc::new(Mutex::new(5));

    let thread_handle = {
        let steps = steps.clone();
        // let steps = Arc::clone(&steps);
        thread::spawn( move|| {
            while *steps.lock().unwrap() > 0{

                sleep(std::time::Duration::from_secs(1));

                println!("Thread step {}", steps.lock().unwrap());

                *steps.lock().unwrap() -= 1;
            }

            // println!("Goodbye!");
            "GoodBye!"
        })
    };
    println!("spawn a thread ! with step count of {}", steps.lock().unwrap());
    std::thread::sleep(std::time::Duration::from_secs(2));

    println!("slept 2 secs ,Now joining thread...");
    let result = thread_handle.join().unwrap();

    println!("thread returned with {}", result);
}
