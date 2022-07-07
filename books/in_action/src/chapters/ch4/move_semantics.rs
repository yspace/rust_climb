
fn use_value(_val: Demo) {
    // ownership transferred to _val
}

struct Demo {
    a: i32,
}

fn main() {
    let demo = Demo{a: 123} ;
    use_value(demo );
    // It's  illegal 
    println!("{}", demo );
}