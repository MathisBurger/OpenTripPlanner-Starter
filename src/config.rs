use serde_derive::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub(crate) struct Config {
    http: HttpConfig,
    path: PathConfig
}

#[derive(Deserialize)]
pub(crate) struct PathConfig {
    data_dir: String,
    graph_dir: String
}
#[derive(Deserialize)]
pub(crate) struct HttpConfig {
    port: u32,
    secure_port: u32
}

pub(crate) fn parse_config() -> Config {
    let contents = fs::read_to_string("./otp-starter.toml").expect("Config file otp-starter.toml does not exist");
    let config: Config = toml::from_str(contents.as_str()).expect("Cannot parse config");
    return config;
}