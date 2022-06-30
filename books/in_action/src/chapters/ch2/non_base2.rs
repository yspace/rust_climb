pub fn main() {
    // 二进制 ob 前缀
    let three = 0b11 ;
    // 八进制 0o 小心 长得太像
    let thirty = 0o36 ; 
    // 十六进制 0x
    let three_hundred = 0x12c ;

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);

    // println!("base 2: {:b} {:b}  ", 20u32,20i8 /*,20f32 */);
}