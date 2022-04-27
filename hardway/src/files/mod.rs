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