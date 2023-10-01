@see 👀 [borrows](https://www.lurklurk.org/effective-rust/borrows.html)

use Rust's smart pointers for interconnected data structures.

[Item 9](https://www.lurklurk.org/effective-rust/references.html) described the most common smart pointer types provided by Rust's standard library.

    Rc allows shared ownership, with multiple things referring to the same item. Often combined with…
    RefCell allows interior mutability, so that internal state can be modified without needing a mutable reference. This comes at the cost of moving borrow checks from compile-time to run-time.
    Arc is the multi-threading equivalent to Rc.
    Mutex (and RwLock) allows interior mutability in a multi-threading environment, roughly equivalent to RefCell.
    Cell allows interior mutability for Copy types.
