pub fn main() {
    using_references();
    needle_in_haystack();
}

fn using_references() {
    let a = 42;
    let r = &a;
    let b = a + *r;

    println!("a + a = {}", b);
}

fn needle_in_haystack() {
    let needle = 0o204;
    let needle = 52;

    let haystack = [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147];

    // Each iteration changes the value of item to refer to the next item within haystack.
    for item in &haystack {
       if *item == needle {
        println!("{}", item);
       }
    }
}
