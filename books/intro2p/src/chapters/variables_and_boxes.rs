
pub fn main() {
    // variables represent memory on the stack, and boxes represent memory on the heap. We will discuss boxes later.

    {
        // let statement
        // The let statement names a variable and binds it to a memory location on the stack. Additionally, this statement 
        // can annotate the type of the variable and initialize its value.
   
        // // Binds a variable
        let x  ;
        x = 1 ;

        let y: i32 ;

        let z: i32 = 1 ;

       // Binds a variable and initialize its value, but lets the compiler infer its type.
        let w = 1 ;

        y = 1 ;
        println!("{}\n{}\n{}\n{}\n " , x, y, z, w) ;
    
        // bind multiple variables and init its value;
        let (x,y,z) = (1,2.0, "hello, world") ;
        assert_eq!(x, 1) ;
        assert_eq!(y, 2.0) ;
        assert_eq!(z, "hello, world") ;
    
    }

    {
        /*
        Assignment and mutability
        If you didn't initialize a variable when binding it, 
        you must assign it a value before reading it.
        The compiler prevents you from using uninitialized variables.
        */
        let x ;
        x = 1 ;
        assert_eq!(x, 1) ;

        let y: i32 ;
        y = 2 ;
        assert_eq!(y, 2) ;


        // variables are immutable by default in Rust. Immutable variables also allows the compiler to optimize the program better. If a program wishes to modify a variable, it must use the mut keyword in the let statement:
        let mut z = 1 ;
        println!("now z is {}", z) ;
        // 
        z = 2 ;
        println!("now z is {}", z) ;
   
        // In a let statement binding multiple variables, mut annotates only the variable following it:

        let (x, mut y, z) =
        (1,2,3) ;
        println!("{}\t{}\t{}\t", x,y,z) ;
   
    }

}