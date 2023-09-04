#[macro_use]
extern crate lazy_static;

use generic::Func;
use wena::*;


mod commands ;
mod settings ;

// mod reject;
mod generic;
// use self::reject::{reject, Rejection};


lazy_static! {
    static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("config can be loaded");
}

fn main() {

    let f = ||{
        println!("hi");
    };

    f.call(()) ;

    // let mut app = Application::new("my-crawler");
    // app.commands([
    //     commands::greet::build_command(CONFIG.clone()),
    // ]);

    // app.run();
}