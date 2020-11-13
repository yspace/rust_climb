

mod ch1 ;
mod ch2 ;

fn main() {
//   ch1::hello::main() ;
//    ch2::vars::main() ;
//    ch2::basic_types::main() ;
//    _seahorse_main();

    _clap_main() ;
}

// ==================================================================
//      ## copy from https://github.com/guoxbin/dtool/blob/master/src/main.rs
mod app;
mod modules;
fn _clap_main(){

    let (app, module_manager) = app::build_app();

    let mut app_clone = app.clone();

    let matches = app.get_matches();

    let (name, matches) = matches.subcommand();

    if let Some(matches) = matches {
        module_manager.run(name, matches);
    } else {
        app_clone.print_help().unwrap_or(());
        println!();
    }
}

fn _seahorse_main(){
    use seahorse::{App,Command,Context};
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
            .usage("cli help(h) [...]")
            .action(|c: &Context| println!("{:?}", c.args)))
        ;

    app.run(args);
}
