// https://blog.madhukaraphatak.com/functional-programming-in-rust-part-1

//  keep lifetime of function as long as value step_value exist. Lifetimes in rust can only exist with references. So in our example we will take &i32 rather than i32.
//  Also we create to reference to Fn using Box.
fn higer_order_fn_return<'a>(step_value:& 'a i32) -> 
                            Box<Fn(i32) -> i32 + 'a > {
       Box::new(move |x:i32| x+step_value)
}