// #[macro_use]

// 导出到crate下
#[macro_export]
macro_rules! my_macros {
    () => {
       println!("hi i am a macro of crate") ;
    };
}

fn init(){
    println!("this is func of mod pub_macros") ;
}

#[macro_export]
macro_rules! do_func_init {
    () => {
       println!("hi i am a macro of mod pub_macros") ;
       // 调用一个init方法 但此方法是在调用者的作用域中定义的 而非本模块下定义的那个init方法 因为展开是在调用者那里进行的
       init() ;
    };
}

#[macro_export]
macro_rules! do_crate_func_init {
    () => {
       println!("hi i am a macro of mod pub_macros") ;
       // 调用一个init方法 此方法是在当前模块的根作用域下定义
       $crate::init() ;
    };
}