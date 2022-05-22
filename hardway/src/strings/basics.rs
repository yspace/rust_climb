
pub fn main(){
    let my_string = String::from("hello world");

    // Length
    println!("Length: {}", my_string.len());
    // is empty
    println!("Is empty: {}", my_string.is_empty());

    for tok in my_string.split_whitespace() {
        println!("{}", tok);
    }

    // contain
    println!("Contains 'world': {}", my_string.contains("world"));

    // push
    let mut s = String::from("foo");
    s.push_str(" bar");
    println!("{}", s);
}