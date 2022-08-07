use std::fs::File;
use std::net::Ipv6Addr ;

// the question mark
// conversion on the
// operation (`?`) implicitly performs a
// error value using the `From` trait
// fn main() -> Result<(), std::io::Error> {
fn main() -> Result<(), Box<dyn Error >> {
    // File::open() returns Result<(), std::io::Error>.
    let _f = File::open("invisible.txt")?;

    // 
// "".parse::<Ipv6Addr>() returns Result<Ipv6Addr, std::net::AddrParseError>.
    let _localhost = "::1"
    .parse::<Ipv6Addr>()?;
    Ok(())
}
