use serde_derive::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub(crate) struct Config {
    pub(crate) http: HttpConfig,
    pub(crate) path: PathConfig,
    pub(crate) memory_limit: String
}

#[derive(Deserialize)]
pub(crate) struct PathConfig {
    pub(crate) data_dir: String,
    pub(crate) graph_dir: String
}
#[derive(Deserialize)]
pub(crate) struct HttpConfig {
    pub(crate) port: u32,
    pub(crate) secure_port: u32
}

/// Parses the config that has been supplied.
/// If the config does not exist an error is thrown
/// An error is also thrown, if there is a missing field in the config
pub(crate) fn parse_config() -> Config {
    let contents = fs::read_to_string("./otp-starter.toml").expect("Config file otp-starter.toml does not exist");
    let config: Config = toml::from_str(contents.as_str()).expect("Cannot parse config");
    return config;
}