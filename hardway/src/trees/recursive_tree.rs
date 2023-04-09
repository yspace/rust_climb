// https://www.eventhelix.com/rust/rust-to-assembly-recursive-tree-fold/

/*

Key takeaways
The Rust compiler optimizes recursive calls to loops only when the recursive calls are tail calls.
When a recursive function calls itself multiple times, the compiler optimizes the last recursive calls to a loop.
Rust allows you to write declarative recursive functions, but it's important to understand the overhead of recursive calls and how they are mapped to loops in certain scenarios.
The Rust compiler opportunistically optimizes away enum discriminators if it can infer the enum variant from the context.


 */
pub enum Tree<T> {
    Node(T, Box<Tree<T>>, Box<Tree<T>>),
    Leaf(T),
}
use Tree::{Leaf, Node};

pub fn sum(tree: &Tree<u64>) -> u64 {
    match tree {
        Leaf(n) => *n,
        Node(n, left, right) => *n + sum(left) + sum(right),
    }
}

// compiler optimizes the recursive factorial call to a loop.
pub fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// the compiler optimizes away one of the two recursive calls to the fibonacci function.
pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}