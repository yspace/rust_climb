extern crate rand;
use std::io;
use rand::Rng ;
use std::cmp::Ordering;

pub fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is: {}",secret_number) ;

    loop {
        println!("please enter your hunch.");

        let mut hunch = String::new() ;
    
        io::stdin().read_line(&mut hunch)
        .ok()
        .expect("Failed to read line") ;
    
        let hunch: u32 = match hunch.trim().parse() {
            Ok(num) => num ,
            Err(_) => continue ,
        } ;
        // .ok()
        // .expect("Please enter a number!") ;
    
        println!("Your hunch was: {}", hunch) ;
    
        match hunch.cmp(&secret_number) {
            Ordering::Less => println!("Very small"),
            Ordering::Greater => println!("Very big!"),
            Ordering::Equal => {
                println!("You won!");
                break ;
            }
        }
    
    }
    
}
