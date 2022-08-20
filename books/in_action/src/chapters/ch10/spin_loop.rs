use std::{thread, time  };          

pub fn main() {
    for n in 1..1001 {
        let mut handlers : Vec<thread::JoinHandle<()>> = Vec::with_capacity(n);

        let start = time::Instant::now();

        for _m in 0..n {
            let handle = thread::spawn(move ||{
                let start = time::Instant::now();
                let pause = time::Duration::from_millis(20) ;

                while start.elapsed() < pause {
                    // NOTE The spin_loop_hint() instruction is not present for every CPU. On platforms that don’t support it, spin_loop_hint() does nothing.
                    // std::sync::atomic::spin_loop _hint(). spin_loop_hint()
                    
                    thread::yield_now() ;
                }
            });
            hadlers.push(handle);
        }

        // A for loop does not permit modifications to the data being iterated over. Instead, the while loop allows us to repeatedly gain mutable access when calling handlers.pop()
        /*
           for handle in &handlers{
                handler.join(); // 
           }
        */
        while let Some(handle) = hadlers.pop() {
            handle.join();
        }

        let finish = time::Instant::now();
        println!("{}\t{:02?}", n, finish.duration_since(start)  );   
    }
}