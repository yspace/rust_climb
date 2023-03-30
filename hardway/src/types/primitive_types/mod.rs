
pub mod integer_types;
pub mod tuples;

// 针对数字类型 rust提供了基础的数学操作(/ * + -) 有checked| unchecked | overflowing | wrapping 不同形式

mod checked_arithmetic{
    // 安全除零
    #[test]
    fn checked_div(){
        assert_eq!((100i32).checked_div(1i32), Some(100i32));
        assert_eq!((100i32).checked_div(0i32), None);
    }
}

#[test]
fn test_me(){

}