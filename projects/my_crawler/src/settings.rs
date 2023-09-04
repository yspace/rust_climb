use std::fmt;
use std::convert::Infallible;


use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub port: u16,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Rule {
    pub name: String,
    pub rule_set: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    // pub rules: Vec<Rule>,
    pub log: Log,
    pub env: ENV,
}

const CONFIG_FILE_PATH: &str = "./config/Default.toml";
const CONFIG_FILE_PREFIX: &str = "./config/";


#[derive(Clone, Debug, Deserialize)]
pub enum ENV {
    Development,
    Testing,
    Production,
}

impl fmt::Display for ENV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ENV::Development => write!(f, "Development"),
            ENV::Testing => write!(f, "Testing"),
            ENV::Production => write!(f, "Production"),
        }
    }
}

impl From<&str> for ENV {
    fn from(env: &str) -> Self {
        match env {
            "Testing" => ENV::Testing,
            "Production" => ENV::Production,
            _ => ENV::Development,
        }
    }
}


impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "Development".into());
        // let mut s = Config::new();
        let mut s = Config::builder().build().unwrap();
        s.set("env", env.clone())?;

        s.merge(File::with_name(CONFIG_FILE_PATH))?;
        s.merge(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))?;

        // This makes it so "EA_SERVER__PORT overrides server.port
        s.merge(Environment::with_prefix("ea").separator("__"))?;

        // s.try_into()
        s.try_deserialize()
    }
}
