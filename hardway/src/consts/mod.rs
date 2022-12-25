const PI: f64 = 3.14159265 ;
// 对于 const ，常量贯穿于整个程序的生命周期。更具体的，Rust 中的常量并没有固定的内存地址。
// 这是因为实际上它们会被 内联 到用到它们的地方。 为此对同一常量的引用并不能保证引用到相同的内存地址。



// 参考 ://www.codercto.com/a/89622.html

pub fn main(){
    println!("PI is {}", PI);

    println!("f64: from {} to {} ",f64::MIN, f64::MAX);

    // let p1 = PI  ;
    // let p2 = PI ;
    // println!("p1: {:p}", &p1);
    // println!("p2: {:p}", &p2);


}