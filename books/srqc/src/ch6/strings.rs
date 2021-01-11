
pub fn main(){
    t_str::main() ;
    t_sring::main() ;
}

mod t_str{
    // 。&str类型是对一块字符串区间的借用，它对所指向的内存空间没有
    //所有权，哪怕&mut str也一样。

    pub fn main(){
        let greeting: &str = "Hello" ;
        let substr : &str = &greeting[2..] ; // 字符串切片类型

        println!("{}", substr) ;

        // &str类型也是胖指针
        is_fat_ptr();
    }

    fn is_fat_ptr(){
        use std::mem ;
        println!("Size of pointer :{}",mem::size_of::<*const ()>()) ;
        println!("Size of &str :{}",mem::size_of::<&str>()) ;
    }


}

mod t_sring{
// String类型。它跟&str类型的主要区别是，它有管理内存
//空间的权力。

    // String类型在堆上动态申请了一块内存空间，它有权对这
    //块内存空间进行扩容，内部实现类似于std：：Vec<u8>类型。所以我们
    //可以把这个类型作为容纳字符串的容器使用

    // 这个类型实现了Deref<Target=str>的trait。所以在很多情况下，
    //&String类型可以被编译器自动转换为&str类型。
    pub fn main(){
        let mut s = String::from("hello") ;
        s.push(' ');
        s.push_str("World.") ;
        println!("{}", s) ;

        //
        fn capitalize(substr: &mut str) {
            // 有权修改 无权扩容或释放内存
            substr.make_ascii_uppercase();
        }
//        fn main() {
            let mut s = String::from("Hello World");
            capitalize(&mut s);
            println!("{}", s);
//        }

    }
}
