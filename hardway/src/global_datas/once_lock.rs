use std::sync::OnceLock;
use std::sync::*;
use std::thread;

// OnceLock is a cell that is assigned once. By definition, after being assigned it can no longer change.
static TEST: OnceLock<Test> = OnceLock::new();

struct Test {
    name: String,
    count: usize,
}

fn init_test() -> Test {
    // Logic to initialize the static data in the OnceLock.
    println!("Initializing test");
    Test {
        name: String::from("BIRD"),
        count: 500,
    }
}

#[test]
fn main() {
    let mut children = vec![];
    for _ in 0..8 {
        children.push(thread::spawn(move || {
            // Initialize the data on the first access.
            // If already initialized, just return the existing data.
            let test = TEST.get_or_init(|| init_test());
            println!("TEST: {}", test.name);
        }));
    }

    for child in children {
        let _result = child.join();
    }
    println!("DONE");
}