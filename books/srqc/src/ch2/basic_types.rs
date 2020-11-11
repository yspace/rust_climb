
pub  fn main(){
    learn_bool();
}
fn learn_bool(){

    let x = true ;
    let y: bool = !x ;

    let z = x && y ;
    println!("{}", z) ;

    let z = x || y ;
    println!("{}", z) ;

    let z = x & y ;
    println!("{}", z);

    let z = x | y ;
    println!("{}", z);

    let z = x ^ y ;
    println!("{}", z);

    logical_op(2,3) ;

    //
    let (a, b) = (3, 5) ;
    if a >= b {
        println!("a>=b");
    }else {
        println!("z<b");
    }

    let mut i = 0 ;
    while i <=10 {
        println!("i: {}",i) ;
        i +=1 ;
    }
}

fn logical_op(x: i32, y: i32){
    let z = x<y ;
    println!("{}<{} => {}",x, y, z) ;
}