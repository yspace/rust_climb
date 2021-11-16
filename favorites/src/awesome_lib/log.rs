// [Rust每周一库: log](https://colobu.com/2019/09/22/rust-lib-per-week-log/)
pub fn act_main(){
// 关于log 库：

    // 和其它语言的日志库一样，它定义了log的级别Level:
    //
    //    Error: 严重错误
    //    Warn: 危险状况
    //    Info: 有用的信息
    //    Debug: 低优先级的调试信息
    //    Trace: 非常低的优先级，很详细的信息

// 一般生产系统我们会把级别设置为Info或者Warn级别，在开发的过程中才设置为Debug甚至Trace级别。

  using_logging() ;
}

use log::{info, trace, warn};

pub fn using_logging() {

    pretty_env_logger::init();

    trace!("a trace log");
    info!("a info long: {}", "abc");
    warn!("a warning log: {}, retrying", "error msg");
    error!("boom!") ;

    println!("using_logging end />>") ;
}
