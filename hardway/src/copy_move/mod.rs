pub fn main() {
    // =   会发生两种情况 或move 或copy

    // 注意 对于复合类型（array tuple ...） copy move 语义由每个成员决定 
    // 具有一票否决特点 整体可copy除非每个成员可copy 只要有一个是动态类型｜不可copy 那么整体是move语义

    // &T 一般是copy语义  &mut T 一般是move语义 
    println!("copy move");

    let a = 1 ;
    let b = a ; // copy
    println!("a is {}", a) ;

    let c = [1, 2, 3, 4] ;
    let c2 = c ; // copy
    println!("c is {:?}", c) ;

    let t = (1, 2.4 , [1,3]) ;
    let t2 = t ; // copy
    println!("t is {:?}", t) ;

    let v = 1 ;
    let b = &v ;
    let b2 = b ; // copy 语义
    println!("b is {:?}", b) ;

}

fn move_semantics(){
    let a = Box::new(1) ;
    let a2 = a ; // move
    // println!("a is {}", a) ;

    let c = ["1".to_string(), "2".to_string()] ;
    let c2 = c ; // move
    // println!("c is {:?}", c);

    let t = (1, 1.2, "hi".to_string()) ;
    let t2 = t ; // move
    // println!("t is {:?}", t) ;

    let mut v = 1 ;
    let b = &mut v ;
    let b2 = b ; // move 语义
    // println!("b is {:?}", b) ;
}