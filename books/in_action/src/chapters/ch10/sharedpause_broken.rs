use std::{thread, time};

pub fn main(){
    let pause = time::Duration::from_millis(20);

    // use move semantics. That, in turn, relies on Copy.
    let handle1 = thread::spawn(move || {
        thread::sleep(pause);
    });
    let handle2 = thread::spawn(move || {
        thread::sleep(pause);
    });

    handle1.join(); 
    handle2.join();


}