use chrono::Local ;

pub fn main() {
    let now = Local::now();
    println!("{}", now) ;
}