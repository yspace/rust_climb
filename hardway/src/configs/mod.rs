use once_cell::sync::Lazy;

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