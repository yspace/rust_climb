use std::fmt;
use std::num::ParseIntError;

use std::fs::File;
use std::io::Write;

#[test]
fn main() {
    println!("{:?}", square("2"));
    println!("{:?}", square("INVALID"));
}
fn square0(val: &str) -> Result<i32, ParseIntError> {
    match val.parse::<i32>() {
        Ok(num) => Ok(i32::pow(num, 2)),
        Err(e) => Err(e),
    }
}

fn square1(val: &str) -> Result<i32, ParseIntError> {
    let num = val.parse::<i32>()?;
    Ok(i32::pow(num, 2))
}

// fn square(val: &str) -> Result<i32, ParseIntError> {
//     let num = val.parse::<i32>()?;
//     let mut f = File::open("fictionalfile.txt")?;
//     let string_to_write = format!("Square of {} is {}", num, i32::pow(num,2));
//     f.write_all(string_to_write.as_bytes())?;
//     Ok(i32::pow(num, 2))
// }

#[derive(Debug)]
pub enum MyError {
    ParseError,
    IOError,
}

impl std::error::Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::ParseError => write!(f, "Parse Error"),
            MyError::IOError => write!(f, "IO Error"),
        }
    }
}

fn square(val: &str) -> Result<i32, MyError> {
    let num = val.parse::<i32>().map_err(|_| MyError::ParseError)?;
    let mut f = File::open("fictionalfile.txt").map_err(|_| MyError::IOError)?;

    let string_to_write = format!("Square of {:?} is {:?}", num, i32::pow(num, 2));
    f.write_all(string_to_write.as_bytes())
        .map_err(|_| MyError::IOError)?;
    Ok(i32::pow(num, 2))
}
