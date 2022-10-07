pub fn run() {
    main_dead_first();
    wait_for_a_thread();
    msg_passing_concurrency() ;
    shared_state_concurrency() ;
    // shared_state_concurrency with arc ;
    with_arc();
}

fn main_dead_first() {
    use std::thread;
    use std::time::Duration;

    thread::spawn(move || {
        for i in 1..10 {
            println!("From spawned thread - {}", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    for i in 1..3 {
        println!("From main thread - {}", i);
        thread::sleep(Duration::from_millis(2));
    }
}

fn wait_for_a_thread() {
    use std::thread;
    use std::time::Duration;

    // 注意 这东西内部可以返回线程的返回值 大部分都是() 但可以试试返回一个东西 在join后处理它
    let h = thread::spawn(|| {
        for i in 0..10 {
            println!("from spawn thread - {}", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    for i in 1..3 {
        println!("from main thread - {}", i);
        thread::sleep(Duration::from_millis(2));
    }
    h.join().unwrap();
}

fn msg_passing_concurrency() {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    let (tx1, rx) = mpsc::channel();
    let tx2 = tx1.clone();

    thread::spawn(move || {
        let num_vec = vec![1, 2, 3];
        for num in num_vec {
            tx1.send(num).unwrap();
            thread::sleep(Duration::from_millis(2));
        }
    });

    thread::spawn(move || {
        let num_vec = vec![4, 5, 6];
        for num in num_vec {
            tx2.send(num).unwrap();
            thread::sleep(Duration::from_millis(2));
        }
    });

    for received_value in rx{
        println!("received :{}", received_value);
    }
}

// shared memory is implemented using Mutex<T> and Arc<T>.
fn shared_state_concurrency() {
    use std::sync::Mutex;

    let mu = Mutex::new(10);
    let mut val = mu.lock().unwrap();
    println!("val = {:?}", val);
    *val += 1 ;
    println!("val = {:?}", val);

    // ## 
    {
        let mu = Mutex::new(10);
        {
        let mut val = mu.lock().unwrap();
         println!("{:?}", mu);
        *val += 1;
        }
        println!("{:?}", mu);
    }
}


fn with_arc() {
    // Arc<T> provides the shared ownership of a value of type T allocated on the heap.
    // Mutex 保证互斥的访问 二者经常配合完成
    use std::sync::{Arc, Mutex};
    use std::thread; 

    let val = Arc::new(Mutex::new(10));
    let mut t_handles = vec![];
    println!("val = {}", *val.lock().unwrap());

    for _ in 0..5{
        let val = val.clone();
        let h = thread::spawn(move ||{
            let mut num  = val.lock().unwrap();
            *num += 1;
        });
        t_handles.push(h);
    }

    for h in t_handles {
        h.join().unwrap();
    }

    println!("val = {}", *val.lock().unwrap()) ;
}