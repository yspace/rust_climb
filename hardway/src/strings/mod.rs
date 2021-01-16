use std::fs::read_to_string;

pub fn main(){
    println!("learn string");

    // let contents = std::fs::read("runtimes/faust.txt").unwrap() ;
    let contents = std::fs::read_to_string("runtimes/faust.txt").unwrap() ;
    println!("contents : {}",contents) ;
}