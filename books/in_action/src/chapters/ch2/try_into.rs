// explicit imports . 解锁类型的一些方法
use std::convert::TryInto ;

pub fn main() {

    let a: i32 = 10 ;
    let b: u16 = 100;

    // try_into() returns a Result type that provides access to the conversion attempt.
    let b_ = b.try_into().unwrap() ;

    if a < b_ {
        println!("Ten is less than one hundred");
    }
}