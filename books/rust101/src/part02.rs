    pub enum SomethingOrNothing<T> {
        Something(T),
        Nothing ,
    }

    use std::ops::Not;

    pub use self::SomethingOrNothing::* ;

    type NumberOrNothing = SomethingOrNothing<i32> ;

    /*
    Notice the syntax for giving generic implementations to generic types:
     Think of the first <T> as declaring a type variable (“I am doing something for all types T”),
      and the second <T> as using that variable (“The thing I do, is implement SomethingOrNothing<T>”).
    */
    impl<T>  SomethingOrNothing<T> {

        fn new(o: Option<T>) -> Self {
            match o {
                None => Nothing,
                Some(t) => Something(t) ,
            }
        }

        fn to_option(self) -> Option<T>{
            match self {
                Nothing => None,
                Something(t) => Some(t),
            }
        }
    }

    fn call_constructor(x: i32)-> SomethingOrNothing<i32> {
        SomethingOrNothing::new(Some(x))
    }

pub trait Minimum: Copy {
    fn min(self, b: Self) -> Self ;
}

pub fn vec_min<T: Minimum>(v: Vec<T>) -> SomethingOrNothing<T>{
    let mut min = Nothing ;

    for e in v {
        min = Something( match min {
            Nothing => e,
            Something(n) => e.min(n)
        }) ;
    }

    min    
}

impl Minimum for i32 {
    fn min(self, b: Self) -> Self  {
        if self < b {self } else { b}
    }
}

impl NumberOrNothing {
    pub fn print(self)  {
        match self {
            Nothing => println!("The number is <nothing>"),
            Something(n) => println!("The number is: {}", n),
        }
    }
}

fn read_vec()-> Vec<i32> {
    vec![18,5,7,3,9]
}
/*
Exercise 02.1: Change your program such that it computes the minimum of a Vec<f32> 
(where f32 is the type // of 32-bit floating-point numbers). 
You should not change vec_min in any way, obviously!

*/
type F32OrNothing = SomethingOrNothing<f32>;
impl Minimum for f32 {
    fn min(self, b: Self) -> Self  {
        if self < b { self } else { b }
    }
}
impl F32OrNothing {
    fn print(self)   {
        match self {
            Nothing => println!("The f32 is <nothing>"),
            Something(n) => println!("The f32 is: {}", n),
        }
    }
}



pub fn main(){
    println!("part02====");
    let vec = read_vec(); 
    let min = vec_min(vec);
    min.print(); 

    // 
    let vec2 = vec![1.0_f32, 2.0 , 0.3 , 0.4] ;
    let min = vec_min(vec2);
    min.print(); 
}