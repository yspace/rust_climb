use std::borrow::Cow ;

// C 风格字符串
use std::ffi::CStr ;

use std::os::raw::c_char ;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

pub fn main() {
    let a = 42 ;

    let b :String ;
    let c: Cow<str>;

    unsafe{
        let b_ptr = &B as *const u8 as *mut u8 ;
        // Conceptually, CStr::from_ptr() takes responsibility for reading the pointer until it reaches 0; 
        // then it generates Cow<str> from the result.
        b =  String::from_raw_parts(b_ptr, 10,10) ;

        let c_ptr = &C as *const u8 as *const c_char ;
        
        c = CStr::from_ptr(c_ptr).to_string_lossy();

        println!("a: {}, b: {}, c: {}", a, b, c);
    }
}