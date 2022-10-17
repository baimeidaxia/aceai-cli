use log::info;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub servers: Vec<ConfigServer>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigServer {
    pub ip: String,
    pub account: String,
    pub password: String,
    pub docker_compose_path: String,
}

impl Config {
    pub fn get_server_by(&self, ip: &String) -> &ConfigServer {
        for ele in &self.servers {
            if ele.ip == *ip {
                return ele.clone();
            }
        }
        panic!("未找到配置")
    }
}

pub fn load() -> Result<Config, serde_yaml::Error> {
    let mut f = File::open("app-conf.yml").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    let config: Config = serde_yaml::from_str(&buf)?;
    info!("load {:?}", config);
    Ok(config)
}
