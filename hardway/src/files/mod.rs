use std::path::PathBuf;

#[test]
fn test_something() {
    // omitted...
    // 获取cargo 的路径
    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push("tests");
    config_path.push("some.conf");
    println!("config file: {:?}", config_path);
    // omitted...

    if cfg!(test) {
        println!("under test");
    } else {
        println!("not test");
    }
}

use std::{ffi::OsStr, fs, path::Path};
fn get_list(action_dir_path: &str) -> Vec<String> {
    fs::read_dir(action_dir_path)
        .unwrap()
        .map(|x| {
            x.unwrap()
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
        .collect()
}

mod v_2 {
    use std::{ffi::OsString, fs, io, path::Path};

    fn get_list(action_dir_path: impl AsRef<Path>) -> io::Result<Vec<OsString>> {
        fs::read_dir(action_dir_path)?
            .map(|entry| entry.map(|e| e.file_name()))
            .collect()
    }
}
