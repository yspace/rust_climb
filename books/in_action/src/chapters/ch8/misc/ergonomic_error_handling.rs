use std::error ;
use std::fmt;
use std::fs::File;
use std::io::Error;
use std::net::AddrParseError;
use std::net::Ipv6Addr;

#[derive(Debug)]
enum UpstreamError {
    IO(std::io::Error),
    Parsing(AddrParseError),
}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl  error::Error for UpstreamError {
    // Because there are default implementations of every method defined by std::error::Error,
    //  we can ask the compiler to do all of the work for us
}

fn main() -> Result<(), UpstreamError> {
    let _f = File::open("invisible.txt")
    /*.mayby_convert_to(UpstreamError); */
    // .map_err(|e| UpstreamError::IO(e))?;
    .map_err( UpstreamError::IO)?;
    let _localhost = "::1".parse::<Ipv6Addr>() 
    /* .mayby_convert_to(UpstreamError);*/
    // .map_err(|e| UpstreamError::Parsing(e))?;
    .map_err( UpstreamError::Parsing)?;

    Ok(())
}
