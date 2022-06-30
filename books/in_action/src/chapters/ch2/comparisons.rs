pub fn main(){
    let a: i32 = 10 ;
    let b: u16 = 100 ;

    // 小类型转换为大类型 是安全的 这被称为`提升`
    if a < (b as i32){
        println!("ten is less than on hundred");
    }

    // 
    assert!(0.1+0.2 == 0.3);
}