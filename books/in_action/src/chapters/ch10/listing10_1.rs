fn
 add(a: i32, b: i32) ->i32 {
    a + b
 }

 pub fn main() {
    let lambda = |a, b| {a+b} ;
    assert_eq!(lambda(4,5,), add(4,5));
    
 }