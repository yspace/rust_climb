use std::env;

fn main() {

    // let manifest_dir_env = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR evironment variable unset");
    // let env_path = Path::new(&manifest_dir_env).join("environment.json");
    // if let Ok(env) = fs::read_to_string(env_path) {
    //     let env: Value = serde_json::from_str(&env).expect("Failed to parse environment.json");
    //     if let Some(env_map) = env.as_object() {
    //         for key in env_map.keys() {
    //             if let Some(env_value) = env[key].as_str() {
    //                 // Simply re-export as an actual environment variable for consistent handling below...
    //                 env::set_var(key, env_value);
    //             }
    //         }
    //     }
    // }

    // @see https://fasterthanli.me/articles/so-you-want-to-live-reload-rust
    // let manifest_dir =
    //     std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir should be set"));
    // let lib_dir = manifest_dir
    //     .parent()
    //     .expect("manifest dir should have a parent");
    // println!("cargo:rustc-link-search={}", lib_dir.display());

    env::set_var("MY_TEMP_VAR", "env_value");

}