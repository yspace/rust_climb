pub mod module;

use std::ffi::OsString;
use std::path::PathBuf;

use clap::{arg, Command};


use module::{App, Module, my_module::MyModule};


fn main() {
    let mut  app = App::new();
    // 为了保证main不经常改动 比如添加了新模块 新功能 可以把变动的代码逻辑写到做了变动的代码附近 做到高内聚
    app.add(MyModule::new()) ;

    let mut commands = app.get_commands();

    // let matches = cli().get_matches();

    let matches = commands.pop().unwrap().get_matches();
    

    match matches.subcommand() {
        
        Some(cmd) => {
            println!("{:?}",cmd)
        }
        
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }

    // Continued program logic goes here...
}