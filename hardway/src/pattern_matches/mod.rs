mod basics;
mod matches;
mod patterns;
mod advanced_matching;
pub mod destructures;

pub fn main() {
    println!("hi  matches!");

    // matches::run() ;
    // patterns::run();
     advanced_matching::run();
}

pub fn is_leap_year(year: i64) -> bool {
    match (year % 4, year % 100, year % 400) {
        (0, 0, 0) => true,
        (0, 0, _) => false,
        (0, _, _) => true,
        (_, _, _) => false,
    }
}