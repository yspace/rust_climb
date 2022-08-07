use std::io::prelude::*;
use std::net::TcpStream ;

pub fn main() -> std::io::Result<()> {
    let host = "www.rustinaction.com:80";

    let mut conn = TcpStream::connect(host)?;

    conn.write_all(b"GET / HTTP/1.0")?;
    // In many networking protocols, \r\n signifies a new line.
    conn.write_all(b"\r\n")?;

    // This may feel redundant given that we used that exact hostname when we connected on lines 7–8. However, one should remembers that the connection is established over IP, which does not have host names. When TcpStream::connect() connects to the server, it only uses an IP address. Adding the Host HTTP header allows us to inject that infor- mation back into the context.
    conn.write_all(b"Host: www.rustinaction.com")?;
    // Two blank new lines signify end of request
    conn.write_all(b"\r\n\r\n")?;

// std::io::copy() streams bytes from a Reader to a Writer.
    std::io::copy(
        &mut conn,
        &mut std::io::stdout()
    )?;
    

    Ok(())
}