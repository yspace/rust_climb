mod bool ;
mod bits ;

pub fn main() {
    bits::run();

    println!("basic data types");
    let text: String = "Hello, World!".to_string(); 
    let my_u32: u32 = 1234;
    let a_char = 'c';
    let byte_buffer = b"raw-byte-buffer";
    let (one, two): (u8, u8) = (1, 2);
    let my_array: [u8; 3] = [1, 2, 3];
    // Nothing is printed, so this just ends
    }