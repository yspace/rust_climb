
/**
 * 
 * The standard library exports a handful of derive macros which we can use to quickly and conveniently impl a trait on a type if all of its members also impl the trait. The derive macros are named after the traits they impl:
    Clone
    Copy
    Debug
    Default
    Eq
    Hash
    Ord
    PartialEq
    PartialOrd

 */

// macro derives Copy & Clone impl for SomeType
#[derive(Copy, Clone)]
struct SomeType;