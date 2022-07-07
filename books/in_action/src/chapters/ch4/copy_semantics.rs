
fn use_value(_val: i32) {
    // copy semantics of Rust's primitive types

}

fn main() {
    let a = 123;
    use_value(a);
    // It's perfectly legal to access after use_value() has returned
    println!("{}", a);
}