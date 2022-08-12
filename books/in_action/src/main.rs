use whale::route::router::Router;

mod routes;

mod chapters;

fn main() {
    // _seahorse_main();
    chapters::ch9::main();
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
        .action(|c| {
            
            // chapters::ch2::grep_lite::main() ;
            // return ;

            let mut router = Router::new();
            routes::configure(&mut router);

            if c.args.len() > 0 {
                let act_key = c.args[0].as_str();
                router.handle(&act_key);

                return;
            }

            println!("Hello, {:?}", c.args);
            router.handle("/");
            
            // println!("{:#?}", router);
            println!("{:#?}", router.descendents(""));
        })
        .command(
            Command::new("help")
                .description("need help?")
                .alias("h")
                .usage("cli help(h) [...]"),
        )
        .command(
            Command::new("ch1")
                .usage("cargo run -p in_action -- ch1")
                .description("rust 第一章")
                .action(|_c: &Context| {
                    chapters::ch1::run();
                }),
        );

    app.run(args);
}
