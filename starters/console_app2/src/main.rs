use wena::*;

 mod commands ;
 mod chapters ;
 
 

fn main() {

    // 配置文件加载
    // 日志组件初始化
    // 各种其他组件创建 如 db之类

    // 构建路由系统 { 注册各个模块的处理器 各个模块只需要一个路由挂节点 }

    Application::new("learn_rust_ab")
        .commands([
            
            commands::to()
            ])
        .run();
}