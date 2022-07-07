#![allow(unused_variables)]

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> CubeSat {
    // ...
    println!("{:?} : {:?}", sat_id, StatusMessage::Ok);
    sat_id
}

pub fn main() {
    let sat_a = CubeSat { id: 0 };
    let sat_b = CubeSat { id: 1 };
    let sat_c = CubeSat { id: 2 };
    /**
     * At each call to check_status(), ownership of one of the instances moves into the function’s local variable sat_id and then returns back to main().
     */
    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);

    // "waiting" ...
    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);
}
