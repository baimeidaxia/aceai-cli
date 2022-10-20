use log::{debug, info};
use reqwest::blocking::Client;
use serde::Deserialize;

use super::conf::Config;

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
    config: &Config,
) -> bool {
    match list_instance(service_name, config) {
        Ok(instances) => {
            if instances.len() == 0 {
                return true;
            }
            for ele in instances {
                if ele.host == *ip && ele.port == port {
                    return false;
                }
            }
            return true;
        }
        Err(e) => panic!("failed is_service_instance_down cause {}", e),
    }
}

pub fn is_service_instance_up(
    service_name: &String,
    ip: &String,
    port: i32,
    config: &Config,
) -> bool {
    match list_instance(service_name, config) {
        Ok(instances) => {
            if instances.len() == 0 {
                return false;
            }
            for ele in instances {
                if ele.host == *ip && ele.port == port {
                    return true;
                }
            }
            return false;
        }
        Err(e) => panic!("failed is_service_instance_up cause {}", e),
    };
}

fn list_instance(
    service_name: &String,
    config: &Config,
) -> Result<Vec<ZuulServiceInstance>, Box<dyn std::error::Error>> {
    let instances = reqwest::blocking::get(format!(
        "{}/nacos/instances?serviceId={}",
        config.zuul_url, service_name
    ))?
    .json::<Vec<ZuulServiceInstance>>()?;
    Ok(instances)
}
