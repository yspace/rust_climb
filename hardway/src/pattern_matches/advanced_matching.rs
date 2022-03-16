fn print_type_of<T>(msg: &str, _: &T) {
    println!("type of {}: {}", msg, std::any::type_name::<T>())
}

macro_rules! print_type {
    ( $x:expr ) => {
        print_type_of(stringify!($x), &$x)
    }
}
fn test() {
    let x = 1;
    print_type!(x);
    println!("x: {}", x);

    let &y = &1;
    print_type!(y);
    println!("y: {}", y);
}

pub fn run() {
    test() ;
}