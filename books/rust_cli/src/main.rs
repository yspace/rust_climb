use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

// cargo run -- main src/main.rs
//  cargo run -p rust_cli -- learnings cargo.toml
fn main() {
    let args = Cli::parse();

    // println!("args: {:?}", args);
    // println!("args: {args:?}" );
    let content =
     std::fs::read_to_string(&args.path).expect("could not read file");
     ; //optimize: use a BufReader instead of read_to_string()

     for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        } 
     }

}
