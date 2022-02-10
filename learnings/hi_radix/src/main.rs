extern crate radix_trie;

use radix_trie::{Trie, TrieCommon};

fn main() {
    let mut trie = Trie::new();
    trie.insert("hello", 19u32);
    trie.insert("hellcat", 35u32);
    trie.insert("not related", 1u32);
    trie.insert("handle nested", 5u32);

    println!("All trie nodes");
    for (k, v) in trie.iter() {
        println!("{}: {}", k, v);
    }

    // println!("All children of 'a'");
    // for n in trie.subtrie(&"h".to_string()).unwrap().children() {
    //     println!("{}: {}", n.key().unwrap(), n.value().unwrap());
    // }

    // println!("{:#?}", trie);
}
