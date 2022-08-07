use std::error::Error;
use std::fs::File;
use std::net::Ipv6Addr;

fn main() -> Result<(), Box<dyn Error>> {
     // File::open() returns Result<(), std::io::Error>.
    let _f = File::open("invisible.txt")?;
    // Error Type is std::net::AddrParseError
    let _localhost = "::1".parse::<Ipv6Addr>()?;

    Ok(())
}
