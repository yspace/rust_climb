
// @see https://blog.logrocket.com/how-to-read-files-rust/
#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Debug, Serialize)]
struct Input {
    xml_file: String,
    json_file: String,
}

#[derive(Deserialize, Debug, Serialize)]
struct Redis {
    host: String,
}

#[derive(Deserialize, Debug, Serialize)]
struct Sqlite {
    db_file: String
}

#[derive(Deserialize, Debug, Serialize)]
struct Postgresql {
    username: String,
    password: String,
    host: String,
    port: String,
    database: String
}

#[derive(Deserialize, Debug, Serialize)]
struct Config {
    input: Input,
    redis: Redis,
    sqlite: Sqlite,
    postgresql: Postgresql
}

// todo: 引入 toml crate
fn main() {
    let config: Config = {
        let config_text = fs::read_to_string("./data/config.toml").expect("LogRocket: error reading file");
        toml::from_str(&config_text).expect("LogRocket: error reading stream")
    };
    println!("[postgresql].database: {}", config.postgresql.database); 

    let _serialized = serde_json::to_string(&config).expect("LogRocket: error serializing to json");
    println!("{}", serialized);
}