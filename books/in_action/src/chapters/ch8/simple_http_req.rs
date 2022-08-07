use std::error::Error ;

use reqwest ;

pub fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.rustinaction.com/";
    let mut response = reqwest::blocking::get(url)?;

    let content = response.text()?;
    println!("{}", content);

    Ok(())

}