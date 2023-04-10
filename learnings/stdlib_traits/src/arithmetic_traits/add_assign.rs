/*

AddAssign<Rhs> types allow us to add + assign Rhs types to them. The trait declaration:

trait AddAssign<Rhs = Self> {
    fn add_assign(&mut self, rhs: Rhs);
}

 */

 use std::ops::AddAssign;

 #[derive(Copy, Clone)]
 struct Point {
     x: i32,
     y: i32
 }
 
 impl AddAssign for Point {
     fn add_assign(&mut self, rhs: Point) {
         self.x += rhs.x;
         self.y += rhs.y;
     }
 }
 
 impl AddAssign<&Point> for Point {
     fn add_assign(&mut self, rhs: &Point) {
         Point::add_assign(self, *rhs);
     }
 } 
 #[test]
 fn main() {
     let mut p1 = Point { x: 1, y: 2 };
     let p2 = Point { x: 3, y: 4 };
     p1 += &p2;
     p1 += p2;
     assert!(p1.x == 7 && p1.y == 10);
 }