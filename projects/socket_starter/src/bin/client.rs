use std::net::{ TcpStream};

fn main() {
    println!("Hello, client!");
    let _stream = TcpStream::connect("localhost:3000").unwrap();
}
