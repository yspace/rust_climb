use tracing::{info, event, instrument, Level} ;
use tracing_subscriber ;

pub fn run(){
    tracing_subscriber::fmt::init() ;

    // 注意用crate:: 做名空间根是访问在挂载在main函数中的模块 而用项目名称做根访问的是lib下的公共可访问结构
    // hardway::tokio_block_on(
    //     foo()
    // );
    hardway::tokio_block_on(
      async{
        println!("in async block");
        // return "nono";

        foo().await ;
      }
    )
}
#[instrument]
async fn foo(){
    println!("this is in async foo");

    let value = 10 ;
    
    event!(Level::INFO, key = value);

    foo2().await;
}
#[instrument]
async fn foo2(){
    println!("this is in async foo2");

    let value = 20 ;

    event!(Level::INFO, key = value);
}