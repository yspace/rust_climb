
fn main() {
     _seahorse_main();


}


fn _seahorse_main(){
    use seahorse::{App,Command,Context,Flag,FlagType};
    use std::env;

    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [args]")
        .action(|_c| println!("Hello, {:?}", c.args))
        .command( Command::new("help")
            .description("need help?")
            .alias("h")
            .usage("cli help(h) [...]")
            .action(|_c: &Context| println!("{:?}", c.args)))

        .command(
            Command::new("std-io")
                .usage("cargo run -p hardway std-io")
                .description("rust的输入和格式化输出")
                .action(|_c: &Context|{
                    use std::io;
                    println!("请输入姓名:");
                    let mut name = String::new();
                    //读取一个字符串
                    io::stdin().read_line(&mut name);
                    //必须使用占位符
                    println!("你好! {}", name);
                })
        )
                

    app.run(args);
}