mod impls ;

pub fn main() {
    impls::main() ;

    let p1 = Point{x:1,y:2} ;
    println!("{:?}", p1);

    let p2 = Point{x:5.3,y:2.1} ;
    println!("{:?}", p2);

    let p3 = Point2{x:12, y:2.1} ;
    println!("{:?}", p3);
}


#[derive(Debug)]
struct Point<T>{
    x: T,
    y: T,
}
#[derive(Debug)]
struct Point2<T,U>{
    x: T,
    y: U,
}