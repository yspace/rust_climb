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
// FIXME 由于有部分代码拷贝自其他项目 语义上Settings 等价与app-config｜ApplicationConfig之类配置语义
mod settings;

// mod reject;
mod generic;
pub mod crawler;
pub mod spiders;
pub mod error;
pub mod constants;
pub mod utils;
mod app;
// FIXME：有机会跟settings合并  config ，service 两个模块拷贝自rbatis样例项目
mod config;
mod service;

// use self::reject::{reject, Rejection};
use generic::Tuple;
use crate::app::Data;

lazy_static! {
    static ref CONFIG: settings::Settings =
        settings::Settings::new().expect("config can't be loaded");
}
// #[tokio::main]

// async fn main() {
fn main() {

    set_var("RUST_LOG", "error");
    env_logger::init();
    //env_logger::builder().format_timestamp(None).init();
    // debug!("this is a debug {}", "message");
    // error!("this is printed by default");

    let my_app_data = Data::new(app::app_data::MyAppData::default());

    // let data = Data::new(Mutex::new(YOUR_DATA))
    // app.app_data(Data::clone(&data));


    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [args]")
        .action(|c| println!("Hello, {:?}", c.args))
        .app_data(CONFIG.clone())
        .app_data(my_app_data.clone())
        .command(commands::greet::build_command(CONFIG.clone()))
        .command(commands::run::build_command(CONFIG.clone()))
        ;

    app.run(args);
}
