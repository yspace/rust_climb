/*

TryFrom and TryInto are the fallible versions of From and Into.

trait TryFrom<T> {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

trait TryInto<T> {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}

Similarly to Into we cannot impl TryInto because its impl is provided by this generic blanket impl:

impl<T, U> TryInto<U> for T
where
    U: TryFrom<T>,
{
    type Error = U::Error;

    fn try_into(self) -> Result<U, U::Error> {
        U::try_from(self)
    }
}

 */

mod demo1 {
    use std::convert::TryFrom;
    use std::error;
    use std::fmt;

    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Debug)]
    struct OutOfBounds;

    impl fmt::Display for OutOfBounds {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "out of bounds")
        }
    }

    impl error::Error for OutOfBounds {}

    // now fallible
    impl TryFrom<(i32, i32)> for Point {
        type Error = OutOfBounds;
        fn try_from((x, y): (i32, i32)) -> Result<Point, OutOfBounds> {
            if x.abs() > 1000 || y.abs() > 1000 {
                return Err(OutOfBounds);
            }
            Ok(Point { x, y })
        }
    }

    // still infallible
    impl From<Point> for (i32, i32) {
        fn from(Point { x, y }: Point) -> Self {
            (x, y)
        }
    }
}

mod demo2 {
    use std::convert::{TryFrom, TryInto};
    use std::error;
    use std::fmt;

    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Debug)]
    struct OutOfBounds;

    impl fmt::Display for OutOfBounds {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "out of bounds")
        }
    }

    impl error::Error for OutOfBounds {}

    impl TryFrom<(i32, i32)> for Point {
        type Error = OutOfBounds;
        fn try_from((x, y): (i32, i32)) -> Result<Self, Self::Error> {
            if x.abs() > 1000 || y.abs() > 1000 {
                return Err(OutOfBounds);
            }
            Ok(Point { x, y })
        }
    }

    struct Triangle {
        p1: Point,
        p2: Point,
        p3: Point,
    }

    impl<P> TryFrom<[P; 3]> for Triangle
    where
        P: TryInto<Point>,
    {
        type Error = P::Error;
        fn try_from([p1, p2, p3]: [P; 3]) -> Result<Self, Self::Error> {
            Ok(Triangle {
                p1: p1.try_into()?,
                p2: p2.try_into()?,
                p3: p3.try_into()?,
            })
        }
    }

    fn example() -> Result<Triangle, OutOfBounds> {
        let t: Triangle = [(0, 0), (1, 1), (2, 2)].try_into()?;
        Ok(t)
    }
}
