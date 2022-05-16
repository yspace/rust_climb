struct Color(u8,u8,u8) ;

pub fn run(){
    let bg = Color(255,70,15);
    println!("{},{},{}", bg.0, bg.1, bg.2);
    let mut bg = Color(255,70,15);
    bg.2 = 45;
    println!("{},{},{}", bg.0, bg.1, bg.2);
}