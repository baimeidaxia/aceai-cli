use log::{debug, info};
use reqwest::blocking::Client;
use serde::Deserialize;

use super::conf::Config;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NacosInstance {
    name: String,
    group_name: String,
    pub hosts: Vec<NacosInstanceHost>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NacosInstanceHost {
    pub instance_id: String,
    pub ip: String,
    pub port: i32,
    pub weight: f32,
    pub healthy: bool,
    pub enabled: bool,
}

pub fn get_service_by(
    service_name: &String,
    config: &Config,
) -> Result<NacosInstance, Box<dyn std::error::Error>> {
    let service = reqwest::blocking::get(format!(
        "{}/nacos/v1/ns/instance/list?serviceName={}",
        config.nacos_url, service_name
    ))?
    .json::<NacosInstance>()?;
    Ok(service)
}

pub fn update_weight_by(
    service_name: &String,
    ip: &String,
    port: i32,
    weight: f32,
    config: &Config,
) -> Result<String, Box<dyn std::error::Error>> {
    let resp = Client::builder()
        .build()?
        .put(format!(
            "{}/nacos/v1/ns/instance?serviceName={}&ip={}&port={}&weight={}",
            config.nacos_url, service_name, ip, port, weight
        ))
        .send()?
        .text()?;
    debug!(
        "service {} update weight to {} response {:?}",
        service_name, weight, resp
    );
    Ok(resp)
}

pub fn is_exist_service_instance(
    service_name: &String,
    ip: &String,
    port: i32,
    config: &Config,
) -> bool {
    match get_service_by(service_name, config) {
        Ok(service) => {
            if service.hosts.len() == 0 {
                return false;
            }
            for ele in service.hosts {
                if ele.ip == *ip && ele.port == port {
                    return true;
                }
            }
            return false;
        }
        Err(e) => panic!("failed is_exist_service_instance cause {}", e),
    }
}
