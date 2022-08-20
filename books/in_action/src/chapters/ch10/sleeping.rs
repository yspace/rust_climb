use std::{thread, time};

pub fn main() {
    let start = time::Instant::now();

    let handler = thread::spawn(move || {
        let pause = time::Duration::from_millis(300);
        thread::sleep(pause.clone());
    });

    /*
    join is an extension of the thread metaphor. When threads are spawned, these are said to have forked from their parent thread. To join threads means to weave these back together again.
In practice, join means wait for the other thread to finish. The join() function instructs the OS to defer scheduling the calling thread until the other thread finishes.

    */
    handler.join().unwrap();

    let finish = time::Instant::now();
    println!("{:02?}", finish.duration_since(start));

}