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


// 相较于常规宏 这种宏方式相当于多出一些匹配分支  这些分支都是内部使用的帮助分支 为主导出的宏做下手的
// 从另一种角度看 就是功能有点不纯！了 索性有特定前缀做区分 还是比较容易区分的 @开头是作为惯例的 主要是防止被常规token-type捕获 
// 但对于大嘴巴的tt类型还是能被吃进去 所以这种分支尽量出现在靠前的位置
#[macro_export]
macro_rules! m_action2 {
    // ============================
    // 此处是内部帮助宏分支 对比上面的情况   相当于把接口 变为一个公共接口 然后在内部再做路由 这样就不需要暴露两个外用接口了
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