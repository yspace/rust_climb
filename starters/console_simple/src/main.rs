//! An example of simple console app in rust.
use std::env;


fn main() {
    let from = if env::args_os().count() == 2 {
        env::args_os().nth(1).unwrap()
    } else {
        println!("Please enter a from path.");
        std::process::exit(1);
    };

     println!("your input : {:?}", from);
}

fn arg1(){
    let from = if env::args_os().count() == 2 {
        env::args_os().nth(1).unwrap()
    } else {
        println!("Please enter a from path.");
        std::process::exit(1);
    };
}

fn arg2(){
    let (from, into) = if env::args_os().count() == 3 {
        (
            env::args_os().nth(1).unwrap(),
            env::args_os().nth(2).unwrap(),
        )
    } else {
        println!("Please enter a from and into path.");
        std::process::exit(1);
    };
}