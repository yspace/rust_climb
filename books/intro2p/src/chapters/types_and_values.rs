
// Type determines how the compiler interprets bytes in memory as values. 
// A type defines a set of valid values and operations on these values. 
pub fn main(){
    atomic_primitive_types();
}

fn atomic_primitive_types(){
    // The compiler implements the Copy trait on these types.

    {
        // The bool type has two values: true and false.
        let b1 = true ;
        let b2 = false ;

        // operations
        println!("!true is : {}", !b1) ;
        println!("true and false is : {}", b1 && b2) ;
        println!("true or false is : {}", b1 || b2) ;
    }

    {
        // Integer types
        /*
        Rust defines the following machine-independent integer types, which contain 1, 2, 4, 8 bytes, respectively.

        Signed integers: i8, i16, i32, i64.
        Unsigned integers: u8, u16, u32, u64.
        Rust also defines the following machine-dependent integer types, whose lengths are large enough to store the addresses of the machine:

        isize: signed integer.
        usize: unsigned integer.

        */
       
    }

    {
        /*
            Floating point types
            f32: 32-bit floating point.
            f64: 64-bit floating point.
        */
    }

    {
        /*
        Textual types
        */

        let c1 = 'a' ;
        let c2 = '秀' ;

        fn print_mem_bytes<T: ?Sized>(val: &T){
           let s = std::mem::size_of_val(val) ;
           println!("size is :{}", s) ;
        }
        print_mem_bytes(&c1) ;
        print_mem_bytes(&c2) ;

    }
}