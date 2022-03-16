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
    let content = std::fs::read_to_string(&args.path).expect("could not read file"); //optimize: use a BufReader instead of read_to_string()

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}

mod example {
    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let result = std::fs::read_to_string("test.txt");
        let content = match result {
            Ok(content) => content,
            Err(error) => {
                return Err(error.into());
            }
        };
        println!("file content: {}", content);
        Ok(())
    }
}

mod question_mark {
    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string("test.txt")?;
        println!("file content: {}", content);
        Ok(())
    }
}

mod providing_context {
    #[derive(Debug)]
    struct CustomError(String);

    fn main() -> Result<(), CustomError> {
        let path = "test.txt";
        let content = std::fs::read_to_string(path)
            .map_err(|err| CustomError(format!("Error reading `{}`: {}", path, err)))?;
        println!("file content: {}", content);
        Ok(())
    }
    mod _anyhow {
        use anyhow::{Context, Result};

        fn main() -> Result<()> {
            let path = "test.txt";
            let content = std::fs::read_to_string(path)
                .with_context(|| format!("could not read file `{}`", path))?;
            println!("file content: {}", content);
            Ok(())
        }
    }
}
