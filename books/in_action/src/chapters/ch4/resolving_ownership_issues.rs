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
    fn connect(&self, sat_id: u64)-> CubeSat{
        CubeSat {
            id: sat_id,
            mailbox: Mailbox {
                messages: Vec::new(),
            },
        }
    }

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

fn fetch_sat_ids()-> Vec<u64>{
    vec![ 1,2, 3]
}



pub fn main() {
    let base = GroundStation{} ;

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids{
        let mut sat = base.connect(sat_id);
        base.send(&mut sat,Message::from("Hello there!"));
    }
}
