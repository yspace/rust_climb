/**
 * Segmentation faults are generated when the CPU and OS detect that your pro- gram is attempting to access memory regions that they aren’t entitled to. Memory regions are divided into segments. 
 */
pub fn main() {
    let mut n_nonzero = 0 ;

    // for i in 0..10000 {
    for i in 1..10000 {
        let ptr = i as *const u8 ; 
        let byte_at_addr = unsafe {*ptr} ;

        if byte_at_addr != 0 {
            n_nonzero += 1 ;
        }
    }

    println!("non-zero bytes in memory: {}", n_nonzero) ;
}