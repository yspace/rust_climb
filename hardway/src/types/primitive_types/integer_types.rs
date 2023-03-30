// i   for signed ;    u for unsigned ;
// 位数：8 | 16 | 32 | 64 | 128

#[test]
fn integer_literals() {
    let value = 0u8;
    println!("value={},len={}", value, std::mem::size_of_val(&value));
    let value = 0b1u16;
    println!("value={},len={}", value, std::mem::size_of_val(&value));
    let value = 0o2u32;
    println!("value={},len={}", value, std::mem::size_of_val(&value));
    let value = 0x3u64;
    println!("value={},len={}", value, std::mem::size_of_val(&value));
    let value = 4u128;
    println!("value={},len={}", value, std::mem::size_of_val(&value));

   // ob1111_1111 0o1111_1111 0x1111_1111 b'A'

}
