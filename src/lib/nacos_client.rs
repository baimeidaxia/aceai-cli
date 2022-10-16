use log::{debug, info};
use reqwest::blocking::Client;
use serde::Deserialize;

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

pub fn get_service_by(service_name: &String) -> Result<NacosInstance, Box<dyn std::error::Error>> {
    let service = reqwest::blocking::get(format!(
        "http://localhost:8848/nacos/v1/ns/instance/list?serviceName={}",
        service_name
    ))?
    .json::<NacosInstance>()?;
    Ok(service)
}

pub fn update_weight_by(
    service_name: &String,
    ip: &String,
    port: i32,
    weight: f32,
) -> Result<(), Box<dyn std::error::Error>> {
    let resp = Client::builder()
        .build()?
        .put(format!(
            "http://localhost:8848/nacos/v1/ns/instance?serviceName={}&ip={}&port={}&weight={}",
            service_name, ip, port, weight
        ))
        .send()?
        .text()?;
    debug!(
        "service {} update weight to {} resposne {:?}",
        service_name, weight, resp
    );
    Ok(())
}

pub fn is_exist_service_instance(
    service_name: &String,
    ip: &String,
    port: i32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let service = get_service_by(service_name)?;
    if service.hosts.len() == 0 {
        return Ok(false);
    }
    for ele in service.hosts {
        if ele.ip == *ip && ele.port == port {
            return Ok(true);
        }
    }
    Ok(false)
}
