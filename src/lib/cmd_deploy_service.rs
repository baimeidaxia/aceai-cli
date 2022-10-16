use std::{thread, time::Duration};

use crate::lib::nacos_client;
use clap::ArgMatches;
use log::info;

use super::{nacos_client::NacosInstanceHost, zuul_client, ssh_client};

pub fn handle(sub_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let service_name = sub_matches.get_one::<String>("service_name").unwrap();
    info!("service {} deploying", service_name);
    let service = nacos_client::get_service_by(&service_name);
    let hosts = service?.hosts;
    info!("service {} has {:?} instances", service_name, hosts.len());
    handle_instances(&service_name, hosts);
    info!("service {} deploy finish", service_name);
    Ok(())
}

fn handle_instances(service_name: &String, hosts: Vec<NacosInstanceHost>) {
    let mut idx = 1;
    let size = hosts.len() as i32;
    for host in &hosts {
        stat_instance(service_name, host, size, idx);
        close_instance_flow(service_name, &host);
        let _ = check_service_status_in_zuul(service_name, &host, String::from("down"));
//        ssh_client::ssh(ip, account, password, cmd)
        let _ = check_service_started_in_nacos(service_name, &host);
        open_instance_flow(service_name, &host);
        let _ = check_service_status_in_zuul(service_name, &host, String::from("up"));
        idx = idx + 1;
    }
}

fn stat_instance(service_name: &String, host: &NacosInstanceHost, instance_size: i32, idx: i32) {
    info!(
        "service {} instance({}/{}) {}:{} stats {{ weight={} healthy={} enable={} }}",
        service_name,
        idx,
        instance_size,
        host.ip,
        host.port,
        host.weight,
        host.healthy,
        host.enabled
    );
}

fn close_instance_flow(service_name: &String, host: &NacosInstanceHost) {
    let ip = &host.ip;
    let port = host.port;
    let _ = nacos_client::update_weight_by(service_name, ip, port, 0.0);
    info!("service {}:{}:{} close flow", service_name, ip, port);
}

fn open_instance_flow(service_name: &String, host: &NacosInstanceHost) {
    let ip = &host.ip;
    let port = host.port;
    let _ = nacos_client::update_weight_by(service_name, ip, port, 1.0);
    info!("service {}:{}:{} open flow", service_name, ip, port);
}

fn check_service_status_in_zuul(
    service_name: &String,
    host: &NacosInstanceHost,
    check_type: String,
) -> Result<(), Box<dyn std::error::Error>> {
    info!(
        "service {}:{}:{} checking is {} in zuul",
        service_name, &host.ip, host.port, check_type
    );

    let mut i = 1;
    let max = 60;
    while i <= max {
        let res = if check_type == "down" {
            zuul_client::is_service_instance_down(service_name, &host.ip, host.port)?
        } else {
            zuul_client::is_service_instance_up(service_name, &host.ip, host.port)?
        };

        if res {
            info!(
                "service {}:{}:{} checked {} status, cost {} seconds",
                service_name, &host.ip, host.port, check_type, i
            );
            break;
        }

        info!(
            "service {}:{}:{} recheck with times {}",
            service_name,
            &host.ip,
            host.port,
            i - 1
        );

        thread::sleep(Duration::from_secs(1));

        i = i + 1;
    }
    Ok(())
}

fn check_service_started_in_nacos(
    service_name: &String,
    host: &NacosInstanceHost,
) -> Result<(), Box<dyn std::error::Error>> {
    info!(
        "service {}:{}:{} checking is started in nacos",
        service_name, &host.ip, host.port
    );

    let mut i = 1;
    let max = 60;
    while i <= max {
        let exist = nacos_client::is_exist_service_instance(service_name, &host.ip, host.port)?;

        if exist {
            info!(
                "service {}:{}:{} checked started, cost {} seconds",
                service_name, &host.ip, host.port, i
            );
            break;
        }

        info!(
            "service {}:{}:{} recheck with times {}",
            service_name,
            &host.ip,
            host.port,
            i - 1
        );

        thread::sleep(Duration::from_secs(1));

        i = i + 1;
    }

    Ok(())
}
