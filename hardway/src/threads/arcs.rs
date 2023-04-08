use std::thread::spawn;
use std::sync::Arc;

#[derive(Debug)]
struct User {
    name: String
}
#[test]
fn main() {
    //  Arc并不会为所包裹的类型带来线程安全功能！User仍旧需要是 Sync and Send
    let user_original = Arc::new(User { name: "drogus".to_string() });

    let user = user_original.clone();
    let t1 = spawn(move || {
        println!("Hello from the first thread {}", user.name);
    });

    let user = user_original.clone();
    let t2 = spawn(move || {
        println!("Hello from the first thread {}", user.name);
    });

    t1.join().unwrap();
    t2.join().unwrap();
}