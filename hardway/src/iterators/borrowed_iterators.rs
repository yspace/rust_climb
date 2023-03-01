use std::collections::HashMap;

fn hashmap() {
    let mut h = HashMap::new();

    h.insert(String::from("Hello"), String::from("World"));

    for (key, value) in h.iter() {
        println!("{}: {}", key, value);
    }
}

fn array() {
    let a = [1, 2, 3];
    for x in a.iter() {
        println!("{}", x);
    }
}
