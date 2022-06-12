use radix_trie::{Trie, TrieCommon};

pub fn run() {
    println!("Hello, radix!");

    let mut t:Trie<&str, fn()> = Trie::new();
    t.insert("hi", || {
        println!("fn: hi");
    });
    t.insert("hi_qing", || {
        println!("fn: hi qing");
    });

    t.insert("hello", || {
        println!("fn: hello");
    });

    t.get("hi").unwrap()();
    t.get("hello").unwrap()();

    println!("all keys:");
    for k in t.keys() {
        println!("{}", k);
    }

   let v =  t.subtrie("hi").unwrap();
    println!("subtrie hi:");
//    println!("{:?}", v);
   for k in v.keys() {
         println!("{}", k);
   }
   v.get("hi").unwrap().unwrap()();
   v.get("hi_qing").unwrap().unwrap()();

}
