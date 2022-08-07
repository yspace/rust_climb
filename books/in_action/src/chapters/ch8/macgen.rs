extern crate rand; //

use rand::RngCore ;
use std::fmt ;
use std::fmt::Display ;

// Uses the newtype pattern to wrap a bare array without any extra overhead
#[derive(Debug)]
struct MacAddress([u8; 6]);

impl Display for MacAddress{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let octet = &self.0;
        // Converts each byte to hexadecimal notation
        write!(f, "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:",
     octet[0], octet[1], octet[2], octet[3], octet[4], octet[5])
    }
}

impl MacAddress {
    fn new() -> MacAddress{
        let mut octets: [u8; 6] = [0; 6];
        rand::thread_rng().fill_bytes(&mut octets);
        // Sets the MAC address to local and unicast
        octets[0] |= 0b_0000_0011;

        // MacAddress(octets)
        Self{ 0: octets}
    }

    fn is_local(&self) -> bool {
        (self.0[0] & 0b_0000_0011) == 0b_0000_0011
    }

    fn is_unicast(&self) -> bool {
        (self.0[0] & 0b_0000_0001) == 0b_0000_0001
    }
}

pub fn main() {
    let mac = MacAddress::new();
    assert!(mac.is_local());
    assert!(mac.is_unicast());
    println!("{}", mac);
}