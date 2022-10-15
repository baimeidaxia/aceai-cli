use crate::lib::nacos_client;
use clap::ArgMatches;
use log::info;

use super::nacos_client::NacosInstanceHost;

pub fn handle(sub_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let service_name = sub_matches.get_one::<String>("service_name").unwrap();
    let service = nacos_client::get_service_by(&service_name);
    let hosts = service?.hosts;
    info!(
        "the {} service has {:?} instances",
        service_name,
        hosts.len()
    );
    handle_instances(&service_name, hosts);
    Ok(())
}

fn handle_instances(service_name: &String, hosts: Vec<NacosInstanceHost>) {
    let mut idx = 1;
    for ele in hosts {
        info!(
            "instance({}) {}:{} weight={} healthy={} enable={}",
            idx, ele.ip, ele.port, ele.weight, ele.healthy, ele.enabled
        );
        let _ = nacos_client::update_weight_by(service_name, &ele.ip, ele.port, 0.0);
        idx = idx + 1;
    }
}
