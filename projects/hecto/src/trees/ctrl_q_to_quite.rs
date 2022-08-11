use std::io::{self,stdout, Read};
use termion::raw::IntoRawMode ;

fn to_ctrl_byte(    c: char ) ->u8 { 
    // When you compare the output for with the output of the key without , 
    // you will notice that Ctrl sets the upper 3 bits to 0

    // The ASCII character set seems to be designed this way on purpose. (It is also similarly designed so that you can set and clear a bit to switch between lowercase and uppercase.
        //  If you are interested, find out which byte it is and what the impact is on combinations such as Ctrl - A in contrast to Ctrl-Shift-A.)
        let byte = c as u8;
        byte & 0b0001_1111 
}

fn main() {
        let _stdout = stdout().into_raw_mode().unwrap();

    for b in io::stdin().bytes() {
       let b = b.unwrap();
       let c = b as char;

       // ASCII codes 0–31 are all control characters, and 127 is also a control character. ASCII codes 32–126 are all printable
       // @see https://www.asciitable.com/
       if c.is_control() {
        println!("{:?} \r", b) ;
       }else{
        //  key combinations seem to map the letters A–Z to the codes 1–26.
        println!("{:?} ({})\r", b,c) ;
       }

        if b == to_ctrl_byte('q') {
            break;
        }
    }


}

