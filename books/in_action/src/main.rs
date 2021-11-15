
mod chapters;

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
        .action(|c| println!("Hello, {:?}", c.args))
        .command( Command::new("help")
            .description("need help?")
            .alias("h")
            .usage("cli help(h) [...]"))
   
        .command(
            Command::new("ch1")
                .usage("cargo run -p hardway strings")
                .description("rust 中字符串的基本使用")
                .action(|_c: &Context|{
                   chapters::ch1::run() ;
                })
        )
        ;

    app.run(args);
}