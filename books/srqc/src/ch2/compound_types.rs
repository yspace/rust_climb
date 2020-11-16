
pub fn main(){
    learn_tuple() ;
}

fn learn_tuple(){
let a = (1i32 , false) ;
    let b = ("a", (1i32), 2i32) ;
    // 元组类型无名称（? 可以起别名)  成员变量通过索引访问

    println!("{:?},{:?}", a, b) ;

    // NOTE 一个元素的元组 跟括号表达式
    let a = (0, ) ; //  这是元组
    let b = (0) ; // 这是括号表达式

    // 元组的访问
    // a: 模式匹配
    let p = (1i32, 2i32) ;
    let (a, b) = p ;
    println!("a=> {}, b=> {}", a, b) ;

    let x = p.0 ;
    let y = p.1 ;
    println!("x:{}, y:{}", x, y) ;

    // 单元类型 unit
    let empty: () = () ;
    // 空元组 跟空结构一样 都是占用0内存空间
    println!("size of a: {}", core::mem::size_of_val(&a)) ;
    println!("size of empty: {}", core::mem::size_of_val(&empty)) ;

    println!("size of i8 {}" , std::mem::size_of::<i8>());
    println!("size of char {}" , std::mem::size_of::<char>());
    println!("size of '()' {}" , std::mem::size_of::<()>());
}

fn learn_struct(){

}