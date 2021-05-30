use std::io;

fn main() {
    println!("Guess the number!");

    println!("please enter your hunch.");

    let mut hunch = String::new() ;

    io::stdin().read_line(&mut hunch)
    .ok()
    .expect("Failed to read line") ;

    println!("Your hunch was: {}", hunch) ;

}
