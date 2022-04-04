#[macro_use]

// 导出到crate下
#[macro_export]
macro_rules! my_macros {
    () => {
       println!("hi i am a macro of crate") ;
    };
}


macro_rules! my_macros2 {
    () => {
       println!("hi i am a macro of mod pub_macros") ;
    };
}