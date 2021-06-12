pub fn main() {
    // variables represent memory on the stack, and boxes represent memory on the heap. We will discuss boxes later.

    {
        // let statement
        // The let statement names a variable and binds it to a memory location on the stack. Additionally, this statement
        // can annotate the type of the variable and initialize its value.

        // // Binds a variable
        let x;
        x = 1;

        let y: i32;

        let z: i32 = 1;

        // Binds a variable and initialize its value, but lets the compiler infer its type.
        let w = 1;

        y = 1;
        println!("{}\n{}\n{}\n{}\n ", x, y, z, w);

        // bind multiple variables and init its value;
        let (x, y, z) = (1, 2.0, "hello, world");
        assert_eq!(x, 1);
        assert_eq!(y, 2.0);
        assert_eq!(z, "hello, world");
    }

    {
        /*
        Assignment and mutability
        If you didn't initialize a variable when binding it,
        you must assign it a value before reading it.
        The compiler prevents you from using uninitialized variables.
        */
        let x;
        x = 1;
        assert_eq!(x, 1);

        let y: i32;
        y = 2;
        assert_eq!(y, 2);

        // variables are immutable by default in Rust. Immutable variables also allows the compiler to optimize the program better. If a program wishes to modify a variable, it must use the mut keyword in the let statement:
        let mut z = 1;
        println!("now z is {}", z);
        //
        z = 2;
        println!("now z is {}", z);

        // In a let statement binding multiple variables, mut annotates only the variable following it:

        let (x, mut y, z) = (1, 2, 3);
        println!("{}\t{}\t{}\t", x, y, z);
    }

    {
        // ## scope
        // When a program rebinds a variable, it frees the memory location 
        // of (and destroys) the current value and then allocates a new memory location for the variable.
        let x = 1 ;
        let p_x = &x as *const i32 ;
        println!("addr: {:?}", p_x) ;

        assert_eq!(x , 1) ;

        let x = "hello world" ;
        let p2_x = &x as *const &str ;
        println!("addr: {:?}", p2_x) ;
        assert_eq!(x , "hello world");

        // If a program wishes to rebind a variable temporarily without destroying its current value, it can introduce a new scope.
        // Parent scope
        let x = 1;
        {
            // `x` in this nested scope shadows `x` in the parent scope.
            let x = "Hello, world";
            assert_eq!(x, "Hello, world");
        }
        assert_eq!(x, 1);
    }

    {
        //Boxes
        // To place a value on the heap, the program creates a box.
        //  This is similar to C's malloc function or C++'s new operator. 
        // In the following example, the program places the value 1 on the heap, 
        // and creates a variable x on the stack to point to the value on the heap.

        let x = Box::new(1) ;
        println!("{:?}" , x) ;
    }
}
