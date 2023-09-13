mod _1{
    fn make_adder(a: i32) -> impl Fn(i32) -> i32 {
        move |b| a + b
    }
    
    #[test]
    fn main() {
        println!("{}", make_adder(1)(2));
    }
}

mod _2{
    fn make_adder(a: i32) -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |b| a + b)
    }
    
    #[test]
    fn main() {
        println!("{}", make_adder(1)(2));
    }
}

mod _3{
    fn counter() -> impl FnMut() -> i32 {
        let mut value = 0;
    
        move || -> i32 {
            value += 1;
            return value;
        }
    }
    
    #[test]
    fn main() {
        let mut incre = counter();
        println!("Count 1: {}", incre());
        println!("Count 2: {}", incre());
    }
}