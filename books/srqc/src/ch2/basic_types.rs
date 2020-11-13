
pub  fn main(){
    learn_bool();
    learn_char();
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

// 学习char类型
fn learn_char(){
    let love =  '❤';
    println!("love char unicode: {}", love) ;

    // 转义
    let c1 = '\n' ;
    let c2 = '\x7f' ; // 8 bit 字符变量
    let c3 = '\u{7FFF}' ; // unicode字符

    // 因为char类型的设计目的是描述任意一个unicode字符，因此它占据
    // 的内存空间不是1个字节，而是4个字节。
}