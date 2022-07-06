// Capital letters in place of a type indicate a generic type. Conventionally, the variables T, U, and V are used as placeholder values, but this is arbitrary.
// E is often used to denote an error type

fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {
    // All of Rust’s operations are defined with trait
    // To reiterate: all of Rust’s operators are syntactic sugar for a trait’s methods
    i + j
}

pub fn main() {
    let res = add(1, 2);
    println!("{}", res);
}

pub mod listing_2_17{
    // 导入 trait|type 到本地scope
    use std::ops::Add;
    use std::time::{Duration};
    
    fn add<T: Add<Output = T>  >(i: T, j: T) -> T {
        i+j
    }

    pub fn main() {
        let floats = add(1.2, 3.4);
        let ints = add(10,20);  

        let durations = add( 
            Duration::new(5,0),
            Duration::new(10,0)
        );


        println!("{}", floats);
        println!("{}", ints);
        println!("{:?}", durations);
    }
}