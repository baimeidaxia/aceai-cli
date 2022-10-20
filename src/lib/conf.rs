use log::{debug, error, info};
use serde::{de::Error, Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use crate::lib::docker_compose_parser;

use super::{docker_compose_parser::DockerComposeService, ssh_client};

const DOCKER_CONFIG_FILE_NAME: &str = "docker_services.json";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub servers: Vec<ConfigServer>,
    pub nacos_url: String,
    pub zuul_url: String,
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

pub fn init() {
    let config = load_global_config();
    // fetch server docker compose file and save to loacl config file
    init_docker_compose_config(&config);
}

pub fn load_global_config() -> Config {
    let mut f = File::open("app-conf.yml").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    match serde_yaml::from_str(&buf) {
        Ok(r) => r,
        Err(e) => panic!("failed load global config cause {}", e),
    }
}

pub fn load_docker_service_configs() -> Vec<DockerComposeService> {
    if !Path::exists(Path::new(DOCKER_CONFIG_FILE_NAME)) {
        error!("the docker config file not exist, please retry");
        return Vec::new();
    }
    let mut f = File::open(DOCKER_CONFIG_FILE_NAME).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    match serde_json::from_str(&buf) {
        Ok(r) => r,
        Err(_) => Vec::new(),
    }
}

pub fn find_docker_service_config<'a>(
    configs: &'a Vec<DockerComposeService>,
    ip: &str,
    port: i32,
) -> &'a DockerComposeService {
    for ele in configs {
        if ele.ip == ip && ele.port == port {
            return ele;
        }
    }
    panic!("not found docker service config by ip/{} port/{}", ip, port);
}

fn init_docker_compose_config(config: &Config) {
    // the docker compose file is exist, return init
    if Path::exists(Path::new(DOCKER_CONFIG_FILE_NAME)) {
        debug!("{} is exist", DOCKER_CONFIG_FILE_NAME);
        return;
    }
    let mut services = Vec::<DockerComposeService>::new();
    for server in &config.servers {
        let files = ssh_client::find_docker_compose_files(&server);
        for file in files {
            info!("server {} docker compose file {}", server.ip, file);
            let content = ssh_client::fetch_docker_compose_content(&server, &file);
            match docker_compose_parser::parse(&content, &file, server) {
                Ok(r) => {
                    for ele in r {
                        services.push(ele);
                    }
                }
                Err(e) => panic!("failed parse docker compose file cause {}", e),
            }
        }
    }

    let mut f = File::create(DOCKER_CONFIG_FILE_NAME).expect("failed to create file");
    let json = serde_json::to_string(&services).unwrap();
    f.write_all(&json.as_bytes()).unwrap();

    info!("saved docker service json file {:?}", services);
}
