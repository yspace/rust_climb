use std::io;
use std::io::prelude::*;

fn read_vec() -> Vec<i32> {
    let mut vec = Vec::<i32>::new();

    let stdin = io::stdin();
    println!("Enter a list of numbers, one per line. End with Ctrl-D (Linux) or Ctrl-Z (Windows).");

    vec
}

pub fn main() {
    let vec = read_vec();
}
