fn vector() {
    let v = vec![1, 2, 3];
    for x in v.into_iter() {
        println!("{}", x);
    }
    // you can't longer use v
}
