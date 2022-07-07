
#[derive(Debug )]
struct CubeSat{
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

impl Copy for CubeSat{}

impl Copy for StatusMessage{}

impl Clone for CubeSat{
    fn clone(&self) -> CubeSat{
        CubeSat{id: self.id}
    }
}
impl Clone for StatusMessage{
    fn clone(&self) -> StatusMessage{
       *self
    }
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    // ...
    StatusMessage::Ok
}

pub fn main() {
    let sat_a = CubeSat { id:0};

    let a_status = check_status(sat_a); 
    println!("a: {:?}", a_status);
   
    let a_status = check_status(sat_a.clone()); 
    println!("a: {:?}", a_status.clone());

    let a_status = check_status(sat_a);
    println!("a: {:?}", a_status);
}