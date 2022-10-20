use log::{debug, info};
use serde::{Deserialize, Serialize};

use super::conf::ConfigServer;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DockerComposeService {
    pub name: String,
    pub yml: String,
    pub ip: String,
    pub port: i32,
}

impl DockerComposeService {
    pub fn new(name: String, yml: String, ip: String, port: i32) -> Self {
        Self {
            name,
            yml,
            ip,
            port,
        }
    }
}

pub fn parse(
    content: &String,
    yml: &String,
    server: &ConfigServer,
) -> Result<Vec<DockerComposeService>, serde_yaml::Error> {
    let node_indent_len = get_node_indent_len(content);
    let mut services = Vec::new();

    let mut name = String::from("");
    for line in content.lines() {
        let line_indent_len = get_indent_len(&line.to_string());
        if line_indent_len == 0 {
            continue;
        }
        if line_indent_len == node_indent_len {
            debug!("find node line {}", line);
            if name != "" {
                debug!("maybe other container {}", name);
                services.push(DockerComposeService::new(
                    name,
                    yml.to_string(),
                    server.ip.to_string(),
                    0,
                ));
            }
            name = line.replace(":", "").trim().to_string();
        }
        if line.contains("SERVER_PORT") {
            let port_str = line.replace("-", "").replace("SERVER_PORT=", "");
            debug!("find port {:?}", port_str.trim());
            let port = port_str.trim().parse().unwrap();
            debug!("{} {}", name, port);
            services.push(DockerComposeService::new(
                name,
                yml.to_string(),
                server.ip.to_string(),
                port,
            ));
            name = String::from("");
        }
    }
    Ok(services)
}

fn get_node_indent_len(content: &String) -> i32 {
    let mut node_indent_len = 0;
    for ele in content.lines() {
        if ele.starts_with(" ") && ele.trim() != "" {
            for (_, &c) in ele.as_bytes().iter().enumerate() {
                if c as char == ' ' {
                    node_indent_len = node_indent_len + 1;
                }
            }
            break;
        }
    }
    node_indent_len
}

fn get_indent_len(line: &String) -> i32 {
    let mut node_indent_len = 0;
    if line.starts_with(" ") && line.trim() != "" {
        for (_, &c) in line.as_bytes().iter().enumerate() {
            if c as char == ' ' {
                node_indent_len = node_indent_len + 1;
            } else {
                break;
            }
        }
    }
    node_indent_len
}
