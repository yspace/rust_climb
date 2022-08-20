use std::io::{self,stdout, Read};
use termion::raw::IntoRawMode ;

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

        if c == 'q'{
            break;
        }
    }


}

