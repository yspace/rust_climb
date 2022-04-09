use clap::{Command, Arg};

#[macro_use]
extern crate log;
extern crate dotenv;
extern crate pretty_env_logger;

use dotenv::dotenv;
use std::env;

// mod 声明
mod awesome_lib;

// cmd: cargo run -p favorites foo yiqing
fn main() {
    dotenv().ok();
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }

    // s function exactly like sub-Commands, because that's exactly what they are. Each
    // instance of a can have its own version, author(s), Args, and even its own
    // subcommands.
    //
    // # Help and Version
    // Just like Commands, each subcommand will get its own "help" and "version" flags automatically
    // generated. Also, like Commands, you can override "-V" or "-h" safely and still get "--help" and
    // "--version" auto generated.
    //
    // NOTE: If you specify a subcommand for your Command, clap will also autogenerate a "help"
    // subcommand along with "-h" and "--help" (commandlies to sub-subcommands as well).
    //
    // Just like arg() and args(), subcommands can be specified one at a time via subcommand() or
    // multiple ones at once with a Vec<> provided to subcommands().
    let matches = Command::new("MyCommand")
        // Normal Command and Arg configuration goes here...
        // In the following example assume we wanted an commandlication which
        // supported an "add" subcommand, this "add" subcommand also took
        // one positional argument of a file to add:
        .subcommand(
            Command::new("add") // The name we call argument with
                .about("Adds files to mycommand") // The message displayed in "mycommand -h"
                // or "mycommand help"
                .version("0.1") // Subcommands can have independent version
                .author("Kevin K.") // And authors
                .arg(
                    Arg::new("input") // And their own arguments
                        // .about("the file to add")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("foo") // The name we call argument with
                .about("sub foo cmd") // The message displayed in "mycommand -h"
                // or "mycommand help"
                .version("0.1") // Subcommands can have independent version
                .author("yiqing.") // And authors
                .arg(
                    Arg::new("to") // And their own arguments
                        //.about("to someone")
                        .index(1)
                        .default_value("some-user-name")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("3rd-log") // The name we call argument with
                .about("facade log lib") // The message displayed in "mycommand -h"
                .author("yiqing."), // And authors
        )
        .subcommand(
            Command::new("3rd-url") //
                .about("url parser") //
                .author("yiqing."), //
        )
        .subcommand(
            Command::new("3rd-reqwest") //
                .about(" HTTP Client for Rust.") //
                .author("yiqing."), //
        )
        .subcommand(
            Command::new("3rd-sysinfo") //
                .about(" HTTP Client for Rust.") //
                .author("yiqing."), //
        )
        .subcommand(
            Command::new("notify") //
                .about(" displaying desktop notifications.") //
                .author("yiqing."), //
        )
        .subcommand(
            Command::new("liquid") //
                .about(" Liquid templating for Rust ") //
                .author("yiqing."), //
        )
        .get_matches();

    // // You can check if a subcommand was used like normal
    // if matches.is_present("add") {
    //     println!("'mycommand add' was run.");
    // }

    // // You can get the independent subcommand matches (which function exactly like Command matches)
    // if let Some(ref matches) = matches.subcommand_matches("add") {
    //     // Safe to use unwrap() because of the required() option
    //     println!("Adding file: {}", matches.value_of("input").unwrap());
    // }

    // // 处理foo子命令
    // if let Some(ref matches) = matches.subcommand_matches("foo") {
    //     // Safe to use unwrap() because of the required() option
    //     println!("hello to: {}", matches.value_of("to").unwrap());
    // }

    // You can also match on a subcommand's name
    match matches.subcommand_name() {
        Some("add") => println!("'mycommand add' was used"),
        Some("3rd-log") => {
            // use awesome_lib ;
            println!("the third vender log library ");

            awesome_lib::log::act_main();
        }
        Some("3rd-url") => {
            // use awesome_lib ;
            println!("the third vender url library ");
        }
        Some("3rd-reqwest") => {
            // use awesome_lib ;
            println!("reqwest lib");
        }
        Some("3rd-sysinfo") => {
            // use awesome_lib ;
            awesome_lib::hello_sysinfo::basic();
        }
        Some("notify") => {
            // use awesome_lib ;
            awesome_lib::notify::run();
        }
        Some("liquid") => {
            // use awesome_lib ;
            awesome_lib::liquid::run();
        }
         

        None => println!("No subcommand was used"),
        _ => println!("Some other subcommand was used"),
    }
}
