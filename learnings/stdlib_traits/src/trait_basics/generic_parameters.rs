trait Trait<'a, T> {
    // signature uses generic type
    fn func1(arg: T);

    // signature uses lifetime
    fn func2(arg: &'a i32);

    // signature uses generic type & lifetime
    fn func3(arg: &'a T);
}

struct SomeType;

impl<'a> Trait<'a, i8> for SomeType {
    fn func1(arg: i8) {}
    fn func2(arg: &'a i32) {}
    fn func3(arg: &'a i8) {}
}

impl<'b> Trait<'b, u8> for SomeType {
    fn func1(arg: u8) {}
    fn func2(arg: &'b i32) {}
    fn func3(arg: &'b u8) {}
}

mod default_value_for_generic_type {
    // make T = Self by default
    trait Trait<T = Self> {
        fn func(t: T) {}
    }

    // any type can be used as the default
    trait Trait2<T = i32> {
        fn func2(t: T) {}
    }

    struct SomeType;

    // omitting the generic type will
    // cause the impl to use the default
    // value, which is Self here
    impl Trait for SomeType {
        fn func(t: SomeType) {}
    }

    // default value here is i32
    impl Trait2 for SomeType {
        fn func2(t: i32) {}
    }

    // the default is overridable as we'd expect
    impl Trait<String> for SomeType {
        fn func(t: String) {}
    }

    // overridable here too
    impl Trait2<String> for SomeType {
        fn func2(t: String) {}
    }
}


mod parameterize_functions{
    trait Trait{
        fn func<'a, T>(t: &'a T);
    }
}

mod _stdlib{
    trait Add {
        type Rhs;
        type Output;
        fn add(self, rhs: Self::Rhs) -> Self::Output;
    }  

    struct Point {
        x: i32,
        y: i32,
    }
    
    impl Add for Point {
        type Rhs = Point;
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
        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 2, y: 2 };
        let p3 = p1.add(p2);
        assert_eq!(p3.x, 3);
        assert_eq!(p3.y, 3);
    }
}

mod parameterized_generic_types{
    trait Add<Rhs> {
        type Output;
        fn add(self, rhs: Rhs) -> Self::Output;
    }
    
    struct Point {
        x: i32,
        y: i32,
    }
    
    impl Add<Point> for Point {
        type Output = Self;
        fn add(self, rhs: Point) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl Add<i32> for Point { // ✅
        type Output = Self;
        fn add(self, rhs: i32) -> Self::Output {
            Point {
                x: self.x + rhs,
                y: self.y + rhs,
            }
        }
    }
    
    #[test]
    fn main() {
        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 2, y: 2 };
        let p3 = p1.add(p2);
        assert_eq!(p3.x, 3);
        assert_eq!(p3.y, 3);
        
        let p1 = Point { x: 1, y: 1 };
        let int2 = 2;
        let p3 = p1.add(int2); // ✅
        assert_eq!(p3.x, 3);
        assert_eq!(p3.y, 3);
    }
}

mod refactor_add_to_generic_type{
    trait Add<Rhs, Output> {
        fn add(self, rhs: Rhs) -> Output;
    }
    
    struct Point {
        x: i32,
        y: i32,
    }
    
    impl Add<Point, Point> for Point {
        fn add(self, rhs: Point) -> Point {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    impl Add<i32, Point> for Point {
        fn add(self, rhs: i32) -> Point {
            Point {
                x: self.x + rhs,
                y: self.y + rhs,
            }
        }
    }
    
    struct Line {
        start: Point,
        end: Point,
    }
    
    impl Add<Point, Line> for Point { // ✅
        fn add(self, rhs: Point) -> Line {
            Line {
                start: self,
                end: rhs,
            }
        }
    }
    #[test]
    fn main() {
        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 2, y: 2 };
        let p3: Point = p1.add(p2);
        assert!(p3.x == 3 && p3.y == 3);
    
        let p1 = Point { x: 1, y: 1 };
        let int2 = 2;
        let p3 = p1.add(int2);
        assert!(p3.x == 3 && p3.y == 3);
    
        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 2, y: 2 };
        let l: Line = p1.add(p2); // ✅
        assert!(l.start.x == 1 && l.start.y == 1 && l.end.x == 2 && l.end.y == 2)
    }
}