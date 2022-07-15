
fn main(){
    // 40 在栈上
    let a: i32 = 40 ;
    // 60 在堆上
    let b: Box<i32> = Box::new(60) ;
    // 为了访问60 我们需要解引用
    println!("{} + {} = {}", a, b, a + *b) ;
}