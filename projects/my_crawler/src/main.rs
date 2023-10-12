#[macro_use]
extern crate lazy_static;

use std::env::set_var;
use log::{debug, error, log_enabled, info, Level};

// 没有用到👇
// use generic::{one, Func, Product};
// use wena::*;

use seahorse::App;
use std::env;

mod commands;
mod settings;

// mod reject;
mod generic;
pub mod crawler;
pub mod spiders;
pub mod error;
pub mod constants;
pub mod utils;
// use self::reject::{reject, Rejection};
use generic::Tuple;

lazy_static! {
    static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("config can be loaded");
}
// #[tokio::main]

// async fn main() {
fn main() {

    set_var("RUST_LOG", "error");
    env_logger::init();
    //env_logger::builder().format_timestamp(None).init();
    // debug!("this is a debug {}", "message");
    // error!("this is printed by default");

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
