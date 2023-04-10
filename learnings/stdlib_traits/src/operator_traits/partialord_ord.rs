/*

enum Ordering {
    Less,
    Equal,
    Greater,
}

trait PartialOrd<Rhs = Self>: PartialEq<Rhs>
where
    Rhs: ?Sized,
{
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>;

    // provided default impls
    fn lt(&self, other: &Rhs) -> bool;
    fn le(&self, other: &Rhs) -> bool;
    fn gt(&self, other: &Rhs) -> bool;
    fn ge(&self, other: &Rhs) -> bool;
}

All PartialOrd impls must ensure that comparisons are asymmetric and transitive. That means for all a, b, and c:

    a < b implies !(a > b) (asymmetry)
    a < b && b < c implies a < c (transitivity)


 */

use std::cmp::Ordering;

fn must_always_agree<T: PartialOrd + PartialEq>(t1: T, t2: T) {
    assert_eq!(t1.partial_cmp(&t2) == Some(Ordering::Equal), t1 == t2);
}

// use std::cmp::Ordering;

// #[derive(PartialEq, PartialOrd)]
#[derive(PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Rhs == Self == Point
impl PartialOrd for Point {
    // impl automatically symmetric & transitive
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(match self.x.cmp(&other.x) {
            Ordering::Equal => self.y.cmp(&other.y),
            ordering => ordering,
        })
    }
}

/*
If all the members of a type impl PartialOrd then it can be derived:

 */

#[derive(PartialEq, PartialOrd)]
enum Stoplight {
    Red,
    Yellow,
    Green,
}

/*

 Ord is a subtrait of Eq and PartialOrd<Self>:

trait Ord: Eq + PartialOrd<Self> {
    fn cmp(&self, other: &Self) -> Ordering;

    // provided default impls
    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;
    fn clamp(self, min: Self, max: Self) -> Self;
}
  */

/*
  The PartialOrd derive macro orders types based on the lexicographical order of their members:

        // generates PartialOrd impl which orders
        // Points based on x member first and
        // y member second because that's the order
        // they appear in the source code
        #[derive(PartialOrd, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        // generates DIFFERENT PartialOrd impl
        // which orders Points based on y member
        // first and x member second
        #[derive(PartialOrd, PartialEq)]
        struct Point {
            y: i32,
            x: i32,
        }

        trait Ord: Eq + PartialOrd<Self> {
            fn cmp(&self, other: &Self) -> Ordering;

            // provided default impls
            fn max(self, other: Self) -> Self;
            fn min(self, other: Self) -> Self;
            fn clamp(self, min: Self, max: Self) -> Self;
        }
   */
