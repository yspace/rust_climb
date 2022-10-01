// use std::path::Path;
use std::path::{Path, PathBuf};

use std::fs;


// @see https://stackoverflow.com/questions/30511331/getting-the-absolute-path-from-a-pathbuf

pub fn run() {
    // 这个是从哪里执行 current_dir 就是哪里 底层libc 的当前文件夹pwd
    println!(
        "current dir {}",
        std::env::current_dir().unwrap().as_path().to_str().unwrap()
    );

    let path = Path::new("downloads/some_date/some_file.txt");
    let display = path.display();

    println!("{}", display);

    let relative_path = PathBuf::from("cargo_home");
    let mut absolute_path = std::env::current_dir().unwrap();
    absolute_path.push(relative_path);
    println!("abs path: {}", absolute_path.as_path().display());


    let srcdir = PathBuf::from("./src");
    println!("{:?}", fs::canonicalize(&srcdir));

    let solardir = PathBuf::from("./../solarized/.");
    println!("{:?}", fs::canonicalize(&solardir));

    // 
    // path-clean 看起来不错
}
