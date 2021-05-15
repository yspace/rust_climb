use whale::util::errors::WhaleResult; // todo 后期给他缩短这个 "面包屑"路径
use whale::core::Context;


// 下面特性是在nightly版本并打开feature gate才可以使用
//#![feature(const_fn)]
//fn main(){
//    use std::sync::atomic::AtomicBool;
//    static FLAG: AtomicBool = AtomicBool::new(true);
//}

// ========================================================
fn cli_cmd_exec_template(){
    // 调用套路
    let ctx = Context::default() ;
    let opts = SomeUsecaseOptions {
        spec: Vec::new() ,
    };
    // 上面这两个参数实际调用中需要从配置文件 或者用户cli输入参数中做转储传递
    some_usecase(&ctx , &opts) ;
}
// 注意在cargo项目中 套路如下：
//. 注意是CliResult  这个是cli相关的跟CargoResult处于不同的位置  CliResult 在六边形架构中是靠外侧UI的东西
//  CmdXxx::exec(config: &mut Config, args: &ArgMatches<'_>) -> CliResult {
//     // 用配置来构造工作空间
//    let ws = args.workspace(config)?;
//
//    // 构造操作需要的参数选项
//    let opts = FetchOptions {
//        config,
//        targets: args.targets(),
//    };
//    // 执行对应的操作方法
//    let _ = ops::fetch(&ws, &opts)?;
//    Ok(())
// }
pub struct SomeUsecaseOptions {
    //  pub config: &'a Config,
    /// A list of input string.
    pub spec: Vec<String>,
}
// 这个是任何框架无关的逻辑
fn some_usecase(ctx: &Context , opts: &SomeUsecaseOptions ) -> WhaleResult<()>{
    println!("hi this is some usecase !");
    Ok(())
}