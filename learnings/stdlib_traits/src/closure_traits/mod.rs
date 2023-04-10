/*

Trait(s) 	Category 	Operator(s) 	Description
Fn 	closure 	(...args) 	immutable closure invocation
FnMut 	closure 	(...args) 	mutable closure invocation
FnOnce 	closure 	(...args) 	one-time closure invocation

trait FnOnce<Args> {
    type Output;
    fn call_once(self, args: Args) -> Self::Output;
}

trait FnMut<Args>: FnOnce<Args> {
    fn call_mut(&mut self, args: Args) -> Self::Output;
}

trait Fn<Args>: FnMut<Args> {
    fn call(&self, args: Args) -> Self::Output;
}

 */

#[test]
 fn test_fn_once() {
    let range = 0..10;
    let get_range_count = || range.count();
    assert_eq!(get_range_count(), 10); // ✅
    // get_range_count(); // ❌
}

#[test]
fn test_fn_mut() {
    let nums = vec![0, 4, 2, 8, 10, 7, 15, 18, 13];
    let mut min = i32::MIN;
    let ascending = nums.into_iter().filter(|&n| {
        if n <= min {
            false
        } else {
            // 注意这里 修改了所捕获的变量值min
            min = n;
            true
        }
    }).collect::<Vec<_>>();
    assert_eq!(vec![0, 4, 8, 10, 15, 18], ascending); // ✅
}

#[test]
fn test_fn() {
    let nums = vec![0, 4, 2, 8, 10, 7, 15, 18, 13];
    let min = 9;
    let greater_than_9 = nums.into_iter().filter(|&n| n > min).collect::<Vec<_>>();
    assert_eq!(vec![10, 15, 18, 13], greater_than_9); // ✅
}

fn add_one(x: i32) -> i32 {
    x + 1
}

#[test]
fn test_function_pointer() {
    let mut fn_ptr: fn(i32) -> i32 = add_one;
    assert_eq!(fn_ptr(1), 2); // ✅
    
    // capture-less closure cast to fn pointer
    fn_ptr = |x| x + 1; // same as add_one
    assert_eq!(fn_ptr(1), 2); // ✅
}

#[test]
fn test_passing_function_pointer (){
    let nums = vec![-1, 1, -2, 2, -3, 3];
    let absolutes: Vec<i32> = nums.into_iter().map(i32::abs).collect();
    assert_eq!(vec![1, 1, 2, 2, 3, 3], absolutes); // ✅
}