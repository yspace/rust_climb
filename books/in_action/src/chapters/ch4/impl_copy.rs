
#[derive(Debug, Copy, Clone)]
struct CubeSat{
    id: u64,
}

#[derive(Debug, Copy, Clone)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    // ...
    StatusMessage::Ok
}

pub fn main() {
    let sat_a = CubeSat { id:0};

    let a_status = check_status(sat_a); 
    println!("a: {:?}", a_status);

    let a_status = check_status(sat_a);
    println!("a: {:?}", a_status);
}