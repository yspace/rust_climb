fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

fn square(value: i32) -> i32 {
    value * value
}

fn cube(value: i32) -> i32 {
    value * value * value
}

pub fn main() {
    println!("apply square: {}", apply(2, square));
    println!("apply cube: {}", apply(2, cube));

    fn add_one(x: i32) -> i32 {
        x + 1
    }
    let result = higer_order_fn(20, add_one);
    println!("higher order function is called by passing a named function as parameter: result: {}", result);
    println!("higher order function is called by passing a anonymous function as parameter: result: {}", 
    higer_order_fn(20, |x:i32| x +1 )
    // Anonymous functions are in rust are part of closure feature of rust. Closures are special functions which has access to their surrounding scope. So anonymous functions are closures with empty scope.


);


}

// where clause in rust is used for type bound on generics
fn higer_order_fn<F>(value: i32, step: F) -> i32
where
    F: Fn(i32) -> i32,
{
    step(value)
}
