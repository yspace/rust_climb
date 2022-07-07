#![allow(unused_variables)]

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

impl CubeSat {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

type Message = String;

struct GroundStation ;

impl GroundStation {
    fn send(&self,to: &mut CubeSat, msg:  Message){
        to.mailbox.messages.push(msg);
    }
}

fn check_status(sat_id: CubeSat) -> CubeSat {
    // ...
    println!("{:?} : {:?}", sat_id, StatusMessage::Ok);
    sat_id
}

fn send(to: &mut CubeSat, msg: Message){
    to.mailbox.messages.push(msg);
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
