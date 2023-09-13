#[macro_use]
extern crate lazy_static;

use generic::{one, Func, Product};
// use wena::*;

use seahorse::App;
use std::env;

mod commands;
mod settings;

// mod reject;
mod generic;
// use self::reject::{reject, Rejection};
use generic::Tuple;

lazy_static! {
    static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("config can be loaded");
}

fn main() {
    // let f = |a,b,c|{
    //     println!("hi");
    //     println!("{} , {} , {}", a, b, c);
    // };

    // f.call((1,2,3)) ;
    // f.call(().combine((1,2,3))) ;
    // f.call(().combine((1,)).combine((2,)).combine((3,))) ;
    // f.call(().combine((2,)).combine(one(3)).combine(one(5))) ;
    // f.call(Product{1,Product{2,3}})

    // let mut app = Application::new("my-crawler");
    // app.commands([
    //     commands::greet::build_command(CONFIG.clone()),
    // ]);

    // app.run();

    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [args]")
        .action(|c| println!("Hello, {:?}", c.args))
        .app_data(CONFIG.clone())
        .command(commands::greet::build_command(CONFIG.clone()))
        .command(commands::run::build_command(CONFIG.clone()))
        ;

    app.run(args);
}
