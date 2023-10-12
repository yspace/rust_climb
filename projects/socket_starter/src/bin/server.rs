use std::net::TcpListener;
// 引入👇两个 读写能力的trait
use std::io::{Read, Write};

fn main() {
    println!("Hello, world!");

    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    println!("Running on port 3000");
    // listener.accept();
    for stream in listener.incoming() {
        // //Read from stream into a bytes buffer (called byte slice)
        // stream.read(&mut [0; 1024]);
        // // construct a message and write to stream
        // let message = "Hello".as_bytes(); // (string slice -> to byte slice )
        // stream.write(message)

        let mut stream = stream.unwrap();
        println!("Connection established");
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
