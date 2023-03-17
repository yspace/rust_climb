use once_cell::sync::Lazy;

use config::Config;

// NOTE: 注意想验证配置是否被加载 运行时当前目录需要在配置文件出现的目录

// #[derive(Parser, Debug, Default, serde::Deserialize, PartialEq)]
#[derive( Debug, Default, serde::Deserialize, PartialEq)]
struct Args {
    log_level: String,
    /// URL for the postgres database
    database_host: String,
    /// PORT number for the database connection database_port: u16,
    /// Database name
    database_name: String,
    /// Web server port
    port: u16,
}

pub fn run() {
    let config = Config::builder()
        .add_source(config::File::with_name("setup"))
        .build()
        .unwrap();

    let config = config.try_deserialize::<Args>().unwrap();

    println!("{:?}", config);
}

// pub static CONFIG: Lazy<config::Config> = Lazy::new(|| {
//     // dev-configuration
//     let mut glob_path = "conf/development/*";
//     let mut settings = Config::default();

//     let run_mode = match std::env::var("RUST_ENV") {
//         Ok(value) => value,
//         Err(_) => String::new(),
//     };

//     if run_mode.eq("production") {
//         glob_path = "conf/production/*";
//         println!("RUST_ENV={}", run_mode);
//     }

//     settings
//         .merge(
//             glob(glob_path)
//                 .unwrap()
//                 .map(|path| File::from(path.unwrap()))
//                 .collect::<Vec<_>>(),
//         )
//         .unwrap();
//     settings
// });
