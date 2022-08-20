use std::thread;


pub fn main() {
    thread::spawn(|| {
        println!("Hello, world!");
    });

    let var1 = 1 ;
    thread::spawn(move || {
        //  
        println!("var1: {}", var1);
    });
}