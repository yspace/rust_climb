struct DynamicBehavior<T> {
    closure: Box<dyn Fn(T) -> T>,
}

impl<T> DynamicBehavior<T> {
    fn new(closure: Box<dyn Fn(T) -> T>) -> Self {
        Self { closure }
    }
    fn run(&self, arg: T) -> T {
        (self.closure)(arg)
    }
}

#[test]
fn it_works() {
    let square = DynamicBehavior::new(Box::new(|num: i64| num * num));
    println!("{} squared is {}", 5, square.run(5))
    
}
