use radix_trie::{Trie, TrieCommon};

pub fn run() {
    println!("Hello, radix!");

    let mut t:Trie<&str, fn()> = Trie::new();
    t.insert("hi", || {
        println!("hi");
    });

    t.insert("hello", || {
        println!("hello");
    });

    t.get("hi").unwrap()();
    t.get("hello").unwrap()();
}
