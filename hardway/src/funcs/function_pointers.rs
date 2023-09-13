fn sum(x: i32, y: i32) -> i32 {
    x + y
}

#[test]
pub fn run() {
    // Explicit coercion to `fn` type is required...
    let op: fn(i32, i32) -> i32 = sum;
    // `fn` types implement `Copy`
    let op1 = op;
    let op2 = op;
    // `fn` types implement `Eq`
    assert!(op1 == op2);
    // `fn` implements `std::fmt::Pointer`, used by the {:p} format specifier.
    println!("op = {:p}", op);
    // Example output: "op = 0x101e9aeb0"
}

#[test]
fn no_explicit_coercion() {
    // 编译器提示的内部类型 不可以协变到 fn sum(i32,i32)->i32
    let op1 = sum;
    let op2 = sum;
    // Both op1 and op2 are of a type that cannot be named in user code,
    // and this internal type does not implement `Eq`.
    // assert!(op1 == op2);
}

// In real code, an `Iterator` method would be more appropriate.
pub fn modify_all(data: &mut [u32], mutator: fn(u32) -> u32) {
    for value in data {
        *value = mutator(*value);
    }
}

#[test]
fn test_simple_mut() {
    fn add2(v: u32) -> u32 {
        v + 2
    }
    let mut data = vec![1, 2, 3];
    modify_all(&mut data, add2);
    assert_eq!(data, vec![3, 4, 5,]);
    
}
