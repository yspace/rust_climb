// @see https://www.eventhelix.com/rust/rust-to-assembly-static-vs-dynamic-dispatch/

// The Shape trait requires that the implementor have a method called `area`
// that returns the area as the associated type `Shape::T`.
pub trait Shape {
    type T;
    fn area(&self) -> Self::T;
}

// A generic Point for a specified type T.
pub struct Point<T> {
    x: T,
    y: T,
}

// A generic Rectangle for a specified type T.
pub struct Rectangle<T> {
    top_left: Point<T>,
    bottom_right: Point<T>,
}

// Implement the Shape trait for the Rectangle using a generic type T.
// The `T` is the return type of the `area` method. The where clause specifies
// that `T` must support subtraction, multiplication and ability to copy.
impl<T> Shape for Rectangle<T>
where
    T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + Copy,
{
    type T = T;
    fn area(&self) -> T {
        let width = self.bottom_right.x - self.top_left.x;
        let height = self.top_left.y - self.bottom_right.y;

        width * height
    }
}

// A function that calculates the area of two shapes and returns a tuple containing the areas.
// This function requires that the two shapes implement the Shape trait. The function will call
// the area function via a static dispatch. Code will be generated for the function only if concrete
// types are specified for `a` and `b`.
pub fn area_pair_static(a: impl Shape<T = f64>, b: impl Shape<T = f64>) -> (f64, f64) {
    (a.area(), b.area())
}

pub fn static_dispatch_pair(a: Rectangle<f64>, b: Rectangle<f64>) -> (f64, f64) {
    // The following line will generate code for the function as concrete types are specified for `a` and `b`.
    area_pair_static(a, b)
}

// This function performs the same function as `area_pair_static` but uses a dynamic dispatch. The compiler
// will generate code for this function. The calls to `area` are made through the `vtable`.
pub fn area_pair_dynamic(a: &dyn Shape<T = f64>, b: &dyn Shape<T = f64>) -> (f64, f64) {
    (a.area(), b.area())
}

// pub fn area_pair_static(a: impl Shape<T = f64>, b: impl Shape<T = f64>) -> (f64, f64) {
//     (a.area(), b.area())
// }

// pub fn static_dispatch_pair(a: Rectangle<f64>, b: Rectangle<f64>) -> (f64, f64) {
//     area_pair_static(a, b)
// }