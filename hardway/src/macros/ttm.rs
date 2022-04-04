#![recursion_limit = "256"]
// 递归限制

macro_rules! min {
    ($e: expr) => {
        $e
    };
    // $e 捕获头 其余的被后面的捕获
    ($e: expr, $($t: tt),+) => {
        // 此处有递归展开了
        std::cmp::min($e, min!($($t),+))
    };
}


#[test]
fn test_min_macro(){
    assert_eq!(min!(1,2,3,4,5),1);
}