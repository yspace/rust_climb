
基础库 
======
其他成员项目来共同依赖他

照抄cargo目录结构


命名
-----
whale  鲸鱼 ：本意是 engine（引擎）  但这个词比较保守
       或者base  
       鲸鱼 大鱼  啥都吃  啥都可以驮着 承载之意  
  
~~~rust


// ========================================================
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
~~~