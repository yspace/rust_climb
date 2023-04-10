/*

trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

 */

use std::ops::Add;

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[test]
fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;
    assert_eq!(p3.x, p1.x + p2.x); // ✅
    assert_eq!(p3.y, p1.y + p2.y); // ✅
}

/*



Within Rust's type system, for some type T, the types T, &T, and &mut T are all treated as unique distinct types
which means we have to provide trait impls for each of them separately.
  */

impl Add for &Point {
    type Output = Point;
    fn add(self, rhs: &Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[test]
fn main_4ref() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = &p1 + &p2; // ✅
    assert_eq!(p3.x, p1.x + p2.x); // ✅
    assert_eq!(p3.y, p1.y + p2.y); // ✅
}

mod _other_output {
    use std::ops::Add;

    #[derive(Copy, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Copy, Clone)]
    struct Line {
        start: Point,
        end: Point,
    }

    // we updated this impl
    impl Add for Point {
        type Output = Line;
        fn add(self, rhs: Point) -> Line {
            Line {
                start: self,
                end: rhs,
            }
        }
    }

    // but forgot to update this impl, uh oh!
    impl Add for &Point {
        type Output = Point;
        fn add(self, rhs: &Point) -> Point {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }
    #[test]
    fn main() {
        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 3, y: 4 };
        let line: Line = p1 + p2; // ✅

        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 3, y: 4 };
        // let line: Line = &p1 + &p2; // ❌ expected Line, found Point
    }
}


mod _dry{
    use std::ops::Add;

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
struct Line {
    start: Point,
    end: Point,
}

// we updated this impl
impl Add for Point {
    type Output = Line;
    fn add(self, rhs: Point) -> Line {
        Line {
            start: self,
            end: rhs,
        }
    }
}
    // updated, DRY impl
impl Add for &Point {
    type Output = <Point as Add>::Output;
    fn add(self, rhs: &Point) -> Self::Output {
        Point::add(*self, *rhs)
    }
}

#[test]
fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let line: Line = p1 + p2; // ✅

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let line: Line = &p1 + &p2; // ✅
}
}