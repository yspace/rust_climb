// prelude imports heavily used traits such as
// Read and Write in I/O operations
use std::io::prelude::* ;

const BYTES_PER_LINE: usize = 16;
// Multiline string literals don’t need double quotes escaped when built with raw string literals (the r prefix and the # delimiters). 
// The additional b prefix indicates that this should be treated as bytes (&[u8]) not as UTF-8 text (&str).
const INPUT: &'static [u8] = br#"
fn main() {
    println!("Hello, world!");
}
"# ;

pub fn main() -> std::io::Result<()> {
    let mut buffer: Vec<u8> = vec![];
    INPUT.read_to_end(&mut buffer)? ;

    let mut position_in_input = 0 ;
    for line in buffer.chunks(BYTES_PER_LINE) {
        print!("[0x{:08x}]", position_in_input) ;

        for byte in line {
            print!("{:02x} ", byte) ;
        }
        println!("") ;
        position_in_input  += BYTES_PER_LINE ;
    }

    Ok(())
}
