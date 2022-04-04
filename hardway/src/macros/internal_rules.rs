#[macro_export]
macro_rules! helper_macro{
    ($a: expr) => {
        println!("{}",$a) ;
    };
}
// 此宏依赖了上面的helper_macro 但为了能被调用又不得不把上面的帮助宏也导出 这样有违封装性 
// 毕竟公共导出的外部接口要尽量少 不用的不要乱导出 容易使调用者迷惑
#[macro_export]
macro_rules! m_action {

    () => {
        println!("hi i am a macro action of crate") ;
        helper_macro!("hello") ;
    };
}

#[macro_export]
macro_rules! m_action2 {
    // ============================
    // 此处是内部帮助宏分支 对比上面的情况   相当于把接口 变为一个公共接口 然后在内部在做路由 这样就不需要暴露两个外用接口了
    (@helper_macro $a: expr) => {
        println!("{}",$a) ;
    };
    // ============================
    () => {
        println!("hi i am a macro action of crate") ;
        // helper_macro!("hello") ;
        m_action2!(@helper_macro "hello i am a helper") ;
    };
}