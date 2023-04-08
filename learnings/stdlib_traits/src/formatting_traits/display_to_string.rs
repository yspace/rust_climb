/*

trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}

*/

use std::fmt;

#[derive(Default)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
#[test]
fn main() {
    println!("origin: {}", Point::default());
    // prints "origin: (0, 0)"

    // get Point's Display representation as a String
    let stringified_point = format!("{}", Point::default());
    assert_eq!("(0, 0)", stringified_point); // ✅
}

// Aside from using the format! macro to get a type's display representation as a String we can use the ToString trait:

/*
trait ToString {
    fn to_string(&self) -> String;
}

generic blanket impl that automatically impls ToString for any type which impls Display:

impl<T: Display + ?Sized> ToString for T;
 */

 #[test] // ✅
 fn display_point() {
     let origin = Point::default();
     assert_eq!(format!("{}", origin), "(0, 0)");
 }
 
 #[test] // ✅
 fn point_to_string() {
     let origin = Point::default();
     assert_eq!(origin.to_string(), "(0, 0)");
 }
 
 #[test] // ✅
 fn display_equals_to_string() {
     let origin = Point::default();
     assert_eq!(format!("{}", origin), origin.to_string());
 }