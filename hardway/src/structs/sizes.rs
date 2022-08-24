use std::mem;
struct Door {
    width: u32,
    height: u32,
    is_open: bool,

    // 大小对齐
    name: String,
}

struct DoorDoor{
    sub_door: Door,
}

pub fn main() {
    println!("size of Door: {} bytes", mem::size_of::<Door>());
    println!("size of the members: {} bytes", mem::size_of::<(u32, u32, bool)>());

    println!("size of a DoorDoor: {} bytes", mem::size_of::<DoorDoor>());
    
    println!("size of String {} bytes", mem::size_of::<String>());
}