mod iterators;

mod asyncs;
mod closures;
mod collections;
mod ecs;
mod errors;
mod funcs;
mod hashmap;
mod pattern_matches;
mod sized;
mod slices;
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
        )
        .command(
            Command::new("sized")
                .usage("cargo run -p hardway sized") //macos: cargo run -p hardway -- sized
                .description("rust sized trait")
                .action(|_c: &Context| {
                    sized::main();
                }),
        )
        .command(
            Command::new("slices")
                .usage("cargo run -p hardway slices") //macos: cargo run -p hardway -- slices
                .description("rust 切片")
                .action(|_c: &Context| {
                    slices::main();
                }),
        )
        .command(
            Command::new("hashmap")
                .usage("cargo run -p hardway hashmap") //macos: cargo run -p hardway -- slices
                .description("rust hashmap")
                .action(|_c: &Context| {
                    hashmap::main();
                }),
        )
        .command(
            Command::new("funcs")
                .usage("cargo run -p hardway -- funcs") //macos: cargo run -p hardway -- slices
                .description("rust funcs")
                .action(|_c: &Context| {
                    funcs::main();
                }),
        )
        .command(
            Command::new("closures")
                .usage("cargo run -p hardway -- closures") //macos: cargo run -p hardway -- slices
                .description("rust closures")
                .action(|_c: &Context| {
                    closures::main();
                }),
        )
        .command(
            Command::new("collections")
                .usage("cargo run -p hardway -- collections") //macos: cargo run -p hardway -- slices
                .description("rust collections")
                .action(|_c: &Context| {
                    collections::main();
                }),
        )
        .command(
            Command::new("pattern_matches")
                .usage("cargo run -p hardway -- pattern_matches") //macos: cargo run -p hardway -- slices
                .description("rust pattern_matches")
                .action(|_c: &Context| {
                    pattern_matches::main();
                }),
        )
        .command(
            Command::new("ecs")
                .usage("cargo run -p hardway -- ecs") //macos: cargo run -p hardway -- slices
                .description("rust ecs leaning")
                .action(|_c: &Context| {
                    ecs::main();
                }),
        )
        .command(
            Command::new("errors")
                .usage("cargo run -p hardway -- errors") //macos: cargo run -p hardway -- slices
                .description("rust error handling")
                .action(|_c: &Context| {
                    errors::main();
                }),
        )
        .command(
            Command::new("asyncs")
                .usage("cargo run -p hardway -- asyncs") //macos: cargo run -p hardway -- slices
                .description("rust async ")
                .action(|_c: &Context| {
                    // https://course.rs/async/intro.html
                    asyncs::main();
                }),
        );

    app.run(args);
}
