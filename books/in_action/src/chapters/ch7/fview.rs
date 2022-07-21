 use std::fs::File ;
use std::io::prelude::* ;
use std::env; 

// Changing this constant changes the program's output
const BYTES_PER_LINE: usize = 16;

const FNAME_INDEX : usize = 2 ;

pub fn main()  {
    println!("{:?}",env::args().collect::<Vec<_>>());

    // let arg1 = env::args().nth(1);
    let arg1 = env::args().nth(FNAME_INDEX);

    let fname = arg1.expect("usage: fview <filename>");
    
    let mut f = File::open(&fname).expect(format!("Failed to open: {}", fname).as_str());
    let mut pos = 0;
    let mut buffer =  [0; BYTES_PER_LINE];

    while let Ok(_) = f.read_exact(&mut buffer){
        println!("[0x{:08x}]", pos);
        for byte in &buffer {
            match *byte {
                0x00 => println!(".    "),
                0xff => println!("##  "),
                _ => print!("{:02x} ", byte),
            }
        }
        println!("");
        pos += BYTES_PER_LINE;
    }



    
}
