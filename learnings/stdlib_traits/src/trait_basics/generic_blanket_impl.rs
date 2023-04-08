/*
A generic blanket impl is an impl on a generic type
instead of a concrete type.
 */

trait Even {
    fn is_even(self) -> bool;
}

impl Even for i8 {
    fn is_even(self) -> bool {
        self % 2_i8 == 0_i8
    }
}

impl Even for u8 {
    fn is_even(self) -> bool {
        self % 2_u8 == 0_u8
    }
}

impl Even for i16 {
    fn is_even(self) -> bool {
        self % 2_i16 == 0_i16
    }
}

// etc

#[test] // ✅
fn test_is_even() {
    assert!(2_i8.is_even());
    assert!(4_u8.is_even());
    assert!(6_i16.is_even());
    // etc
}

mod __ {
    use std::convert::TryInto;
    use std::fmt::Debug;
    use std::ops::Rem;

    trait Even {
        fn is_even(self) -> bool;
    }

    // generic blanket impl 
    // Unlike default impls, which provide an impl, generic blanket impls provide the impl, so they are not overridable.
    impl<T> Even for T
    where
        T: Rem<Output = T> + PartialEq<T> + Sized,
        u8: TryInto<T>,
        <u8 as TryInto<T>>::Error: Debug,
    {
        fn is_even(self) -> bool {
            // these unwraps will never panic
            self % 2.try_into().unwrap() == 0.try_into().unwrap()
        }
    }

    #[test] // ✅
    fn test_is_even() {
        assert!(2_i8.is_even());
        assert!(4_u8.is_even());
        assert!(6_i16.is_even());
        // etc
    }
}
