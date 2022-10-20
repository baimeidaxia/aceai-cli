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
    let mut find_port = false;
    let mut name = String::from("");
    for line in content.lines() {
        let line_indent_len = get_indent_len(&line.to_string());
        if line_indent_len == 0 {
            continue;
        }
        if line_indent_len == node_indent_len {
            debug!("find node line {}", line);
            name = line.replace(":", "").trim().to_string();
        }
        if find_port {
            let port_str = line.replace("-", "");
            let port_vec: Vec<&str> = port_str.trim().split(":").collect();
            debug!("port_port_vec {:?}", port_vec);
            let port = port_vec[0].parse().unwrap();
            debug!("{} {}", name, port);
            services.push(DockerComposeService::new(
                name,
                yml.to_string(),
                server.ip.to_string(),
                port,
            ));
            name = String::from("");
            find_port = false;
        }
        if line.contains("ports:") {
            find_port = true;
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
