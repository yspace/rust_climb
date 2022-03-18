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

    // showing_progress::main();
    logging::main();
    // using_dotenv::main();
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
                // Context trait can be used to add a description. Additionally, it also keeps the original error, so we get a “chain” of error messages pointing out the root cause.
                .with_context(|| format!("could not read file `{}`", path))?;

            println!("file content: {}", content);
            Ok(())
        }
    }
}

mod printing_performances {
    pub fn run() {
        use std::io::{self, Write};

        let stdout = io::stdout(); // get the global stdout entity
        let mut handle = io::BufWriter::new(stdout); // optional: wrap that handle in a buffer
        writeln!(handle, "foo: {}", 42); // add `?` if you care about errors here
    }
}

mod showing_progress {

    pub fn main() {
        let pb = indicatif::ProgressBar::new(100);
        for i in 0..100 {
            std::thread::sleep(std::time::Duration::from_millis(100));
            // do_hard_work();
            pb.println(format!("[+] finished #{}", i));
            pb.inc(1);
        }
        pb.finish_with_message("done");
    }
}
mod logging {
    use dotenv::dotenv;
    use std::env;

    use log::{info, warn};

    pub fn main() {
        dotenv().ok();
        env_logger::init();

        info!("starting up");
        warn!("oops, nothing implemented!");
    }
}

mod using_dotenv {
    extern crate dotenv;

    use dotenv::dotenv;
    use std::env;

    pub fn main() {
        dotenv().ok();

        for (key, value) in env::vars() {
            println!("{}: {}", key, value);
        }
    }
}

// cargo test  -p rust_cli 
#[test]
fn check_answer_validity() {
    fn answer()->usize {
        42
    }
    assert_eq!(answer(), 42);
}