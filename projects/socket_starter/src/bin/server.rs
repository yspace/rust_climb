use std::net::TcpListener;

fn main() {
    println!("Hello, world!");

    let listener = TcpListener::bind("127.0.0.1:3000")
    .unwrap();

    println!("Running on port 3000");
    // listener.accept();
    for stream in listener.incoming() {
        // //Read from stream into a bytes buffer (called byte slice)
        // stream.read(&mut [0; 1024]);
        // // construct a message and write to stream
        // let message = "Hello".as_bytes(); // (string slice -> to byte slice )
        // stream.write(message)

        let _stream = stream.unwrap();
        println!("Connection established");
    }
}
