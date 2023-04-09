// @see https://www.eventhelix.com/rust/rust-to-assembly-return-impl-fn-vs-dyn-fn/

/**
 * Note that move keyword before the returned closure signals to the compiler that a, b, and c should be moved into the closure environment. 
 * If we don't use the move keyword, the closure will borrow the variables a, b, and c immutably. Borrowing the variables a, b, and c immutably is not possible because the closure is returned from the function and they will be dropped when the function returns.
 */
pub fn make_quadratic(a: f64, b: f64, c: f64) -> impl Fn(f64) -> f64 {
    move |x| a*x*x + b*x + c
}

pub fn call_make_quadratic(x: f64) -> f64 {
    let quad_fn = make_quadratic(5.0, 4.0, 3.0);
    quad_fn(x) 
}

/*
In this case, the closure is allocated on the heap and a Box is returned.

Box<dyn Fn> is a fat pointer that really is a tuple of the closure environment and the vtable pointer for the type. The closure environment contains the captured variables a, b, and c. The vtable pointer is a pointer to the vtable for the trait Fn(f64) -> f64. The vtable contains the function pointers for the trait methods. In this case, the vtable contains the function pointer for the method call defined by the Fn trait. The vtable also contains the call_mut and call_once function pointers. In this case, the call_mut and call_once function pointers are essentially the same as the call function pointer.

The vtable also contains the destructor function pointer. The destructor function is called when the Box is dropped. The vtable also saves the size and alignment of the closure environment. The size and alignment are used when the Box is dropped.

The compiler passes each trait object pointer via two registers. One register carries the address of the object. The other register carries the address of the vtable of the object. 

Key Takeaways
The Rust compiler captures the environment and stores it in a closure environment struct.
If a closure is returned as impl Fn, the closure environment is stored on the stack and a thin pointer is returned to the caller.
In many cases the compiler completely inlines the closure and the closure environment is not stored on the stack.
If a closure is returned as a Box<dyn Fn>, the closure environment is stored on the heap and a fat pointer is returned to the caller. The fat pointer contains the address of the closure environment and the address of the vtable.
The vtable contains the destructor for the closure environment, the size and alignment of the closure environment, and the call method for the closure.

 */
pub fn make_quadratic_box(a: f64, b: f64, c: f64) -> Box<dyn Fn(f64) -> f64> {
    Box::new(make_quadratic(a, b, c))
}