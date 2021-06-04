mod iterators;

mod strings;
mod threads;

fn main() {
    _seahorse_main();
}

fn _seahorse_main() {
    use seahorse::{App, Command, Context, Flag, FlagType};
    use std::env;

    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [args]")
        .action(|c| println!("Hello, {:?}", c.args))
        .command(
            Command::new("help")
                .description("need help?")
                .alias("h")
                .usage("cli help(h) [...]"),
        )
        .command(
            Command::new("itr")
                .description("iterators?")
                .usage("cargo run -p hardway itr")
                .action(|_c: &Context| {
                    println!("------------iterators -------------");
                    iterators::main();
                }),
        )
        .command(
            Command::new("std-io")
                .usage("cargo run -p hardway std-io")
                .description("rust的输入和格式化输出")
                .action(|c: &Context| {
                    use std::io;
                    println!("请输入姓名:");
                    let mut name = String::new();
                    //读取一个字符串
                    io::stdin().read_line(&mut name);
                    //必须使用占位符
                    println!("你好! {}", name);
                }),
        )
        .command(
            Command::new("threads")
                .usage("cargo run -p hardway threads")
                .description("rust 中线程的基本使用")
                .action(|c: &Context| {
                    threads::main();
                }),
        )
        .command(
            Command::new("strings")
                .usage("cargo run -p hardway strings") //macos: cargo run -p hardway -- strings
                .description("rust 中字符串的基本使用")
                .action(|_c: &Context| {
                    strings::main();
                }),
        );

    app.run(args);
}
