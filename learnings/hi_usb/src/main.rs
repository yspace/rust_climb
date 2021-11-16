use serialport::* ;

fn main() {

    match available_ports(){
        Ok(ok_val) => println!("OK: {:?}", ok_val) ,
        Err(err_val) => println!("Err")
    }

    println!("Hello, world!");
}
