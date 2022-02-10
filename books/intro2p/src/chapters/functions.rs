pub fn main() {
    {
        // A function takes zero or more input values and optionally returns an output value.
        // By convention, we call the input values parameters in the callee, and arguments in the caller.

        fn foo() {}
        fn bar(x: i32, y: &str) {}
        fn baz(mut x: i32) -> i32 {
            x += 1;
            x
        }
        foo();
        bar(1, "hello world");
        assert_eq!(baz(1), 2);
    }
    {
        // Return value
        fn f() -> i32 {
            1
        }

        fn g(i: bool) -> i32 {
            if i {
                1
            } else {
                -1
            }
        }
    }

    {
        // statement vs expression

        let x;
        // The value of `y` is `()` instead of `1`.
        let y = (x = 1);
        assert_eq!(y, ());
    }

    function_type();
}

type ModMainFunc = fn();
type ModMainFunc2 = fn() -> ();

fn some_main() {
    println!("this is a main demo");
}

fn function_type() {
    /*
    The function type, fn(), is another primitive type. A function type
    specifies the types of
    all the parameters and that of the return value, if any
    The value of a function type contains the pointer to a function.
    */

    fn square(x: i32) -> i32 {
        x * x
    }

    let f: fn(i32) -> i32;
    f = square;
    assert_eq!(f(2), 4);

    fn print(x: &str) {
        println!("{}", x);
    }

    let g: fn(&str) = print;
    g("hello");

    //
    let mf: ModMainFunc2 = some_main; // move is not happened here ! it is function pointer
    mf();
    // you can still call the some_main;
    some_main();
}
