
#[derive(Debug)]
struct MyStruct{
    x: usize,
}
#[derive(Clone ,Copy,Debug)]
struct MyStruct2{
    x: usize,
}

#[test]
pub fn run(){
    let s1 = MyStruct{x: 0};
    let s2 = MyStruct2{x:1} ;

    let s1_2 = s1 ; // 此处move
    let s2_2 = s2 ; // 此处copy-trait发生作用 隐式执行clone方法 
    // println!("{:?}", s1);
    println!("{:?}", s2);

    // debug_my_struct(s1); // 此处亦发生move 但由于前面 = 已经move了所以注释打开会报错 
    debug_my_struct(s2); // s2实现了copy 所以默认都是copy了
    debug_my_struct(s2); // s2实现了copy 所以默认都是copy了

    debug_my_struct2(&s2) ;
}

fn debug_my_struct(s: impl std::fmt::Debug){
    println!("{:?}", s)
}
fn debug_my_struct2(s: &impl std::fmt::Debug){
    println!("{:?}", s)
}