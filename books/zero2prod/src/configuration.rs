use config::Config;
use std::path::{Path, PathBuf};

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut config_path;
    let pb = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // config_path = pb.join("tests").join("config.toml");
    config_path = pb.join("src").join("configuration.yaml");

    // if cfg!(test) {
    //     let pb = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     // config_path = pb.join("tests").join("config.toml");
    //     config_path = pb.join("src").join("dddconfiguration.yaml");
    // } else {
    //     let file = file!();
    //     config_path = Path::new(&file)
    //         .parent()
    //         .unwrap()
    //         .join("configuration.yaml");
    //     // let config_file_name = pb.to_str().unwrap();
    // }

    let mut settings = Config::builder()
        // .add_source(config::File::with_name("configuration"))
        // .add_source(config::File::with_name(config_file_name))
        .add_source(config::File::from(config_path))
        .build()
        .unwrap();
    settings.try_deserialize()
}
