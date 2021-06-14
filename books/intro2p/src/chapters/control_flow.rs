
pub fn main(){
    // Rust provides two types of expressions for control flows: branching expressions and loop expressions.

    branching_expr::run() ;

    loop_expr::run();

}

mod branching_expr{

    pub fn run(){
        if_expr() ;
    }

    fn if_expr(){
        let x = 1 ;
        let y ;

        if x >= 0 {
            y = x ;
        }else {
            y = -x ;
        }
        assert!(y >= 0) ;

        // expression has value! two branches should have the same type! 
        let x = 1 ;
        let y = if x >= 0 {
            x
        }else{
            // if else branch is missing , the value is unit type `()`
            -x
        } ;

        assert!(y >= 0) ;
    }

}

mod loop_expr{
    // for nested loop  you can use label to break outter loop .
    pub fn run(){

        let mut x = 0 ;
        loop {
            x += 1 ;
            if x > 9 {
                break ;
            }
        }
        assert_eq!(x, 10) ;

        // expression
        let mut x = 0 ;
        let y = loop {
            x += 1 ;
            if x > 9 {
                break x ;
            }
        } ; // `;` 
        assert_eq!(y, 10) ;

        // 
        while_expr();

        // 
        for_expr();
    }

    fn while_expr(){
        // The while expression takes a predicate and executes its block indefinitely as long as the predicate is true.

        let mut x = 0 ;
        while x < 10{
            x +=1 ;
        }
        assert_eq!(x, 10) ;

        // continue , break
        let mut x = 0;
        while x < 10 {
            x += 1 ;
            if x %2 == 0 {
                continue ;
            }
            println!("{}", x) ;
        }
        assert_eq!(x, 10) ;
    }

    fn for_expr(){
       // The most common, versatile loop expression is the for expression. A for expression loops over a collection of items.
       
       // means it is associated with collection .

        for  i in 0..10 {
            println!("{}", i) ;
        }
        /*
         * 0..10 is syntactic sugar for the Range type, which implements the std::iter::IntoIterator trait. In fact, a program may use the for expression on any expression that implements the std::iter::IntoIterator trait.
         */

         let a = [1,2,3] ;
         let v = vec![1,2,3] ;
         for i in a.iter() {
             println!("{}", i) ;
         }

         for i in &v {
             println!("{}", i);
         }

         // prints both the index and value.
         for (idx, val) in v.iter().enumerate() {
             println!("{} : {}", idx, val);
         }

    }
}