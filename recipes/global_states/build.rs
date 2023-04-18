use std::{env, path::PathBuf};
// build.rs
// use cc::Build;

fn main() {
    let project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    // let native = project_dir.join("native");

    println!("from build.rs :{}", project_dir.as_path().display());

    // Build::new()
    //     .include(&native)
    //     .file(native.join("stateful.cpp"))
    //     .cpp(true)
    //     .flag_if_supported("-std=c++17")
    //     .compile("stateful");
}