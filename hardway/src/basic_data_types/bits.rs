// https://blog.csdn.net/htyu_0203_39/article/details/105999553
pub fn run(){

    let mut b1:u8  = 0b0000_0001 ;

    b1 |= 0b0000_0010 ;
    println!("ob:{:08b}", b1);

    // unset
    b1 &= 0b1111_0111 ;
    println!("ob:{:08b}", b1);

    // toggle a bit
    b1 ^= 0b0000_1000 ;
    println!("toggle 1:{:08b}", b1);
    b1 ^= 0b0000_1000 ;
    println!("toggle bit:{:08b}", b1);

    b1 = !b1 ;
    println!("flip bits:{:08b}", b1);

    b1 <<= 1 ;
    println!("shift lft:{:08b}", b1);

    b1 >>= 1 ;
    println!("shift rgt:{:08b}", b1);

    // 数字类型也有大量内置方法 包括位操作相关的
    b1 = b1.rotate_left(1);
    println!("rotate lft:{:08b}", b1);


}

// ==    为bit操作定义宏
#[macro_export]
macro_rules! eq_1{
    (&n: expr, &b: expr) =>{
        &n & (1<< &b) != 0
    }
}
#[macro_export]
macro_rules! eq_0{
    (&n: expr, &b: expr) =>{
        &n & (1<< &b) == 0
    }
}
#[macro_export]
macro_rules! set{
    (&n: expr, &b: expr) =>{
        &n | (1<< &b) 
    }
}
#[macro_export]
macro_rules! clr{
    (&n: expr, &b: expr) =>{
        &n & !(1<< &b) 
    }
}

// 有用的crate：
// bitflags : map struct to bit flag set

//  enumflags2::BitFlags;

// 寄存器操作：bounded_registers