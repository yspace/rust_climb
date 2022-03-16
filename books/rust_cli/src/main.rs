struct Cli {
    pattern: String,
    path: std::path::PathBuf, // PathBuf is like a String but for file system paths that work cross-platform
}

fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    let args = Cli {
        pattern: pattern,
        path: std::path::PathBuf::from(path),
    };

    println!("Hello, world!");
}
