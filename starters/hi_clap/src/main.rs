use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let cmd = clap::Command::new("cargo")
        .bin_name("cargo")
        .subcommand_required(true)
        .subcommand(
            clap::command!("example").arg(
                clap::arg!(--"manifest-path" <PATH>)
                    .value_parser(clap::value_parser!(std::path::PathBuf)),
            ),
        );
    let matches = cmd.get_matches();
    let matches = match matches.subcommand() {
        Some(("example", matches)) => matches,
        _ => unreachable!("clap should ensure we don't get here"),
    };
    let manifest_path = matches.get_one::<std::path::PathBuf>("manifest-path");
    println!("{:?}", manifest_path);
}

struct App {
    name: String,
    cmd: clap::Command,
}

impl App {
    pub fn new(name: String) {

        //   let cloned_name = name.clone();
        // let cmd = clap::Command::new(name.as_str())
        //     // .bin_name(cloned_name)
        //     .subcommand_required(true);
        //     // .subcommand(
        //     //     clap::command!("example").arg(
        //     //         clap::arg!(--"manifest-path" <PATH>)
        //     //             .value_parser(clap::value_parser!(std::path::PathBuf)),
        //     //     ),
        //     // );
        //     Self { name: cloned_name.into(), cmd: cmd }
    }
}

impl Module for App {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

pub trait Module {
    fn get_name(&self) -> String;
}
