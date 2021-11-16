// Type determines how the compiler interprets bytes in memory as values.
// A type defines a set of valid values and operations on these values.
pub fn main() {
    atomic_primitive_types();
    composite_primitive_types();
}

fn atomic_primitive_types() {
    // The compiler implements the Copy trait on these types.

    {
        // The bool type has two values: true and false.
        let b1 = true;
        let b2 = false;

        // operations
        println!("!true is : {}", !b1);
        println!("true and false is : {}", b1 && b2);
        println!("true or false is : {}", b1 || b2);
    }

    {
        // Integer types
        /*
        Rust defines the following machine-independent integer types, which contain 1, 2, 4, 8 bytes, respectively.

        Signed integers: i8, i16, i32, i64.
        Unsigned integers: u8, u16, u32, u64.
        Rust also defines the following machine-dependent integer types, whose lengths are large enough to store the addresses of the machine:

        isize: signed integer.
        usize: unsigned integer.

        */
    }

    {
        /*
            Floating point types
            f32: 32-bit floating point.
            f64: 64-bit floating point.
        */
    }

    {
        /*
        Textual types
        */

        let c1 = 'a';
        let c2 = '秀';

        fn print_mem_bytes<T: ?Sized>(val: &T) {
            let s = std::mem::size_of_val(val);
            println!("size is :{}", s);
        }
        print_mem_bytes(&c1);
        print_mem_bytes(&c2);

        // ## str
        let s1 = "hi china !";
        let s0 = String::from("你好  世界");
        let s2 = &s0;
        println!("{}", s1);
        println!("{}", s2);
        fn str_fn(s: &str) {
            println!("str_fn: params is : {}", s);
        }
        str_fn(&s1);
        str_fn(&s2);
    }
}

fn composite_primitive_types() {
    // Composite primitive types contain other types.
    {
        // Arrays and slices
        // Array [T; N]: contains N elements of the type T. For example:

        let a1 = [0i32; 1];
        println!("{:?}", a1);

        let a2: [&str; 2] = ["hello", "world"];
        println!("{:#?}", a2);

        // index operation
        assert_eq!(["Hello", "World"][1], "World");

        // Slice &[T]: contains a pointer to a range of elements in an array
        //  whose elements have the type T. You may create a slice from an array
        // or another slice. For example:
        /*
        // A slice containing the elements (start, start + 1, ..., end - 1) in `array`
            &array[start..end]
            // Same as `&array[0..end]`
            &array[..end]
            // Same as `&array[start..array.len()]`
            &array[start..]
            // Same as &array[0..array.len()]`
            &array[..]
            // Same as &array[..]`
        */
        let a = ["hello", "world"];
        let mut s1 = &a[..];
        println!("{:?}", s1);
        let a2 = ["hello ", "rust", "world"];
        s1 = &a2[..2];
        println!("{:?}", s1);
        s1 = &a2[1..3];
        println!("{:?}", s1);
    }

    {
        // The tuple type (T1, T2, ..., Tn) contains a sequence of elements where each element may be a different type
        // Empty tuple
        let _ = ();
        // Single-element tuple must end with `,` to distinguish it from a parenthesized expression
        let _ = (1,);
        // A tuple of two elements
        let t = (1, "a");

        println!("( {} , {}   )", t.0, t.1);

        // The empty tuple (), also called the "unit type", is special. It has a single value () and is used to represent "no value". E.g., when an expression or function returns no value, it returns ().

        assert_eq!({}, ());

        /*
        Properties of composite primitive types
        Two array types [T1; N1] and [T2; N2] are considered to be identical if T1==T2 and N1==N2. Two tuple types (A1, A2, ..., An) and (B1, B2, ..., Bn) are considered to be identical if A1==B1, ..., An==Bn.

        Composite primitive types automatically implement the Copy trait if all their constituent types implement the Copy trait.
        */
    }
}
