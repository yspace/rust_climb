// @see https://itsallaboutthebit.com/arc-mutex/

use std::thread;

#[derive(Debug)]
struct User {
    name: String,
}

#[test]
fn main() {
    let user = User {
        name: "drogus".to_string(),
    };
    // Scoped threads is a feature that allows creating a thread bound to a scope, thus allowing compiler to ensure no of the threads outlives the scope.
   
    /*
    The way scoped threads work is all of the threads created in the scope are guaranteed to be finished before the scope closure finishes.
    Or in other words

before the scoped closure goes out of scope, the threads are joined and awaited to be finished. 
Thanks to that the compiler knows that none of the borrows will outlive the owner.
     */
    thread::scope(|s| {
        s.spawn(|| {
            println!("Hello from the first thread {}", &user.name);
        });

        s.spawn(|| {
            println!("Hello from the second thread {}", &user.name);
        });
    });
}