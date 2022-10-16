use log::{debug, info};
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ZuulServiceInstance {
    service_id: String,
    host: String,
    port: i32,
}

pub fn is_service_instance_down(
    service_name: &String,
    ip: &String,
    port: i32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let instances = list_instance(service_name)?;
    if instances.len() == 0 {
        return Ok(true);
    }
    for ele in instances {
        if ele.host == *ip && ele.port == port {
            return Ok(false);
        }
    }
    Ok(true)
}

pub fn is_service_instance_up(
    service_name: &String,
    ip: &String,
    port: i32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let instances = list_instance(service_name)?;
    if instances.len() == 0 {
        return Ok(false);
    }
    for ele in instances {
        if ele.host == *ip && ele.port == port {
            return Ok(true);
        }
    }
    Ok(false)
}

fn list_instance(
    service_name: &String,
) -> Result<Vec<ZuulServiceInstance>, Box<dyn std::error::Error>> {
    let instances = reqwest::blocking::get(format!(
        "http://localhost:9000/nacos/instances?serviceId={}",
        service_name
    ))?
    .json::<Vec<ZuulServiceInstance>>()?;
    Ok(instances)
}
