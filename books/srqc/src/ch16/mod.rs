pub fn main(){
    let v1 = 1 ;
    let p = &v1; // 取引用
    let v2 = *p ; // 解引用
    println!("{}, {}", v1, v2) ;

    //
    auto_deref()
}

fn auto_deref(){
    let s = "hello" ;
    println!("len: {}", s.len());
    println!("len: {}", (&s).len);
    println!("len: {}", (&&&&&&&&&&s).len());
}