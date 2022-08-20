use std::{thread, time};

pub fn main() {
    let start = time::Instant::now();

    let handler = thread::spawn(move || {
        let pause = time::Duration::from_millis(300);
        thread::sleep(pause.clone());
    });
    let handler2 = thread::spawn(move || {
        let pause = time::Duration::from_millis(300);
        thread::sleep(pause.clone());
    });

    
    handler.join().unwrap();
    handler2.join().unwrap();

    let finish = time::Instant::now();
    println!("{:?}", finish.duration_since(start));

}