pub fn main() {
    // f32 没有实现 std::fmt::Binary 
    let a: f32 = 42.42;
    // 但f32 -> i32|u32    
    let frankentype: u32 = unsafe { 
        // 让Rust解释一个f32为u32值 不影响底层的位表示
        std::mem::transmute(a) };
    println!("{}", frankentype);
    // {:032b} 表示通过 std::fmt::Binary trait来格式化一个二进制 用32个零在左侧填充
    /*
    The 032 reads as “left-pad with 32 zeros” and the right-hand b invokes the std::fmt::Binary trait.
    This contrasts with the default syntax ({}), 
    which invokes the std::fmt ::Display trait, 
    or the question mark syntax ({:?}), 
    which invokes std::fmt:: Debug.

    */
    println!("{:032b}", frankentype); 

    let b: f32 = unsafe {
        std::mem::transmute(frankentype)
    };
   println!("{}", b);
   assert_eq!(a, b);
}
