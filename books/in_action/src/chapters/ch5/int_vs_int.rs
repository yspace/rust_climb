pub fn main(){
    // 数据类型决定了位序列表示什么

    let a: u16 = 50115 ;
    let b: i16 = -15421 ;

    // 相同的位模式 但不同的类型
    println!("{:016b} {}", a,a);
    println!("{:016b} {}", b,b); 
}