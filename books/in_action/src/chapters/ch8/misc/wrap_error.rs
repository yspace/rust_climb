use std::fmt;
use std::fs::File;
use std::io;
use std::net;
use std::net::Ipv6Addr;

#[derive(Debug)]
enum UpstreamError {
    IO(io::Error),
    Parsing(net::AddrParseError),
}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for UpstreamError {
    // Because there are default implementations of every method defined by std::error::Error,
    //  we can ask the compiler to do all of the work for us
}

fn main() -> Result<(), UpstreamError> {
    // It’s also possible to remove the calls to map_err(). But to enable that, we need to implement From

    let _f = File::open("invisible.txt").map_err(UpstreamError::IO)?;
    let _localhost = "::1".parse::<Ipv6Addr>().map_err(UpstreamError::Parsing)?;

    Ok(())
}
