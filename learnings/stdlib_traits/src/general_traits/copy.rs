/*
trait Copy: Clone {}


We copy Copy types, e.g. T -> T. Copy promises the copy operation will be a simple bitwise copy so it will be very fast and efficient. We cannot impl Copy ourselves, only the compiler can provide an impl, but we can tell it to do so by using the Copy derive macro, together with the Clone derive macro since Copy is a subtrait of Clone:

#[derive(Copy, Clone)]
struct SomeType;

Copy refines Clone. A clone may be slow and expensive but a copy is guaranteed to be fast and cheap, so a copy is just a fast clone. If a type impls Copy that makes the Clone impl trivial:

// this is what the derive macro generates
impl<T: Copy> Clone for T {
    // the clone method becomes just a copy
    fn clone(&self) -> Self {
        *self
    }
}
 */