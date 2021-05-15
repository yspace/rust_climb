pub fn main(){
    let v1 = 1 ;
    let p = &v1; // 取引用
    let v2 = *p ; // 解引用
    println!("{}, {}", v1, v2) ;
}