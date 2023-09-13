use std::{sync::mpsc, thread};

#[test]
fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("hello here!".to_owned()).unwrap();
        println!("sent msg from thread!");
    });

    let msg = rx.recv().unwrap();
    println!("main thread got message {}", msg);

    // let msg = rx.recv().unwrap(); // 因为tx已经出了作用域（droped） 再次recv会导致panic
}

mod send_multiple_messages {

    use std::{sync::mpsc, thread, time::Duration};

    #[test]
    fn main() {
        // tx is clonable !
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            // 直接发送 &str 也可以不用to_owned
            tx.send("hello here!".to_owned()).unwrap();
            println!("sent msg from thread!");
            thread::sleep(Duration::from_millis(1000));
            tx.send("hello here2!".to_owned()).unwrap();
        });

        // while let <pattern>  = <expression> { ... }
        while let Ok(msg) = rx.recv() {
            println!("[main thread] got mst: {}", msg);
        }

        // let msg = rx.recv().unwrap(); // 因为tx已经出了作用域（droped） 再次recv会导致panic
    }
}

mod from_many_thread {

    use std::{sync::mpsc, thread, time::Duration};

    #[test]
    fn main() {
        // tx is clonable !
        let (tx, rx) = mpsc::channel();

        for thread_idx in 0..10 {
            let tx_clone = tx.clone();
            thread::spawn(move || {
                for i in 0..10 {
                    let msg = format!("msg {} from thead {}!", i, thread_idx);
                    tx_clone.send(msg).unwrap();
                }
            });
        }

        //  主tx 生命周期跟rx一样 所以rx.recv 不会得到Err的 除非如下👇手动drop
        drop(tx);
        // while let <pattern>  = <expression> { ... }
        while let Ok(msg) = rx.recv() {
            println!("[main thread] got msg: {}", msg);
        }
    }
}

// https://github.com/CS128-Honors-Illinois/sp23-lecture-code/tree/main/lecture13

mod for_real {
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn main() {
        let chunk_size = 10_000;
        let num_threads = 10;

        let max_data = chunk_size * num_threads;
        let data = (0..max_data).collect::<Vec<usize>>();

        let (tx, rx) = mpsc::channel();

        for i in 0..num_threads {
            // GET SUB-VECTORS OF SIZE 10,000
            let start = i * chunk_size;
            let end_excl = start + chunk_size;
            let owned_subvec = data[start..end_excl].to_vec();

            let tx_clone = tx.clone();

            thread::spawn(move || {
                let min = owned_subvec[0];
                let max = owned_subvec[owned_subvec.len() - 1];

                let sub_vec_sum: usize = owned_subvec
                    .into_iter()
                    .map(lecture13::expensive_computation)
                    .sum();
                println!("Subvec sum from {min} to {max} is {sub_vec_sum}");
                tx_clone.send(sub_vec_sum).unwrap();
            });
        }

        drop(tx);
        let mut total = 0;
        while let Ok(value) = rx.recv() {
            println!("Receiver got {value}!");
            total += value;
        }

        // assert_eq!(total, data.into_iter().sum());
        println!("Total sum is: {}", total);
    }

    mod lecture13 {
        use std::thread;
        use std::time::Duration;

        pub fn expensive_computation<T>(x: T) -> T {
            thread::sleep(Duration::from_micros(100));
            x
        }
    }
}
