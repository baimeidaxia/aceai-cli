use std::{
    thread::{self, sleep},
    time::Duration,
};

use crate::lib::nacos_client;
use clap::ArgMatches;
use indicatif::ProgressBar;
use log::{debug, info};

use super::{
    conf::{self, Config, ConfigServer},
    docker_compose_parser::DockerComposeService,
    nacos_client::NacosInstanceHost,
    ssh_client, zuul_client,
};

pub fn handle(sub_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let service_name = sub_matches.get_one::<String>("service_name").unwrap();
    info!("service {} deploying", service_name);
    let config = conf::load_global_config();
    let service = nacos_client::get_service_by(&service_name, &config);
    let hosts = service?.hosts;
    info!("service {} has {:?} instances", service_name, hosts.len());
    let _ = handle_instances(&service_name, hosts, &config);
    info!("service {} deploy finish", service_name);
    Ok(())
}

fn handle_instances(
    service_name: &String,
    hosts: Vec<NacosInstanceHost>,
    config: &Config,
) -> Result<(), serde_yaml::Error> {
    let docker_service_configs = conf::load_docker_service_configs();

    let mut idx = 1;
    let size = hosts.len() as i32;
    for host in &hosts {
        let server = config.get_server_by(&host.ip);
        debug!("server {:?}", server);
        stat_instance(service_name, host, size, idx);
        close_instance_flow(service_name, &host, config);
        let _ = check_service_status_in_zuul(service_name, &host, String::from("down"), config);
        deploy_docker(&docker_service_configs, host, server);
        let _ = check_service_started_in_nacos(service_name, &host, config);
        open_instance_flow(service_name, &host, config);
        let _ = check_service_status_in_zuul(service_name, &host, String::from("up"), config);
        idx = idx + 1;
    }
    Ok(())
}

fn stat_instance(service_name: &String, host: &NacosInstanceHost, instance_size: i32, idx: i32) {
    info!(
        "service {} instance({}/{}) {}:{} stats {{ weight={} healthy={} enable={} }} staring",
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

fn close_instance_flow(service_name: &String, host: &NacosInstanceHost, config: &Config) {
    let ip = &host.ip;
    let port = host.port;
    let resp = nacos_client::update_weight_by(service_name, ip, port, 0.0, config);
    info!("close nacos flow response {:?}", resp);
}

fn open_instance_flow(service_name: &String, host: &NacosInstanceHost, config: &Config) {
    let ip = &host.ip;
    let port = host.port;
    let resp = nacos_client::update_weight_by(service_name, ip, port, 1.0, config);
    info!("open nacos flow resposne {:?}", resp);
}

fn check_service_status_in_zuul(
    service_name: &String,
    host: &NacosInstanceHost,
    check_type: String,
    config: &Config,
) {
    info!("check until the status is {} in zuul", check_type);

    let pb = ProgressBar::new_spinner();

    let mut i = 1;
    let max = 60;
    while i <= max {
        let res = if check_type == "down" {
            zuul_client::is_service_instance_down(service_name, &host.ip, host.port, config)
        } else {
            zuul_client::is_service_instance_up(service_name, &host.ip, host.port, config)
        };

        if res {
            break;
        }

        pb.inc(1);
        pb.set_message(format!("checking {}s", i));
        thread::sleep(Duration::from_secs(1));
        i = i + 1;
    }

    pb.finish_with_message("done");
}

fn check_service_started_in_nacos(
    service_name: &String,
    host: &NacosInstanceHost,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("check until service is started in nacos");

    let pb = ProgressBar::new_spinner();

    let mut i = 1;
    let max = 60;
    while i <= max {
        let exist =
            nacos_client::is_exist_service_instance(service_name, &host.ip, host.port, config);

        if exist {
            break;
        }

        pb.inc(1);
        pb.set_message(format!("checking {}s", i));
        thread::sleep(Duration::from_secs(1));

        i = i + 1;
    }
    pb.finish_with_message("done");

    Ok(())
}

fn deploy_docker(
    docker_service_configs: &Vec<DockerComposeService>,
    host: &NacosInstanceHost,
    server: &ConfigServer,
) {
    let docker_service_config =
        conf::find_docker_service_config(&docker_service_configs, host.ip.as_str(), host.port);
    debug!("find docker_service_config {:?}", docker_service_config);

    info!("deploy docker with name {}", docker_service_config.name);

    let pb = ProgressBar::new_spinner();

    let resp = ssh_client::ssh_shell(
        &server.ip,
        &server.account,
        &server.password,
        &format!(
            "cd {}; docker compose -f {} pull {}; docker compose -f {} up -d {}",
            server.docker_compose_path,
            docker_service_config.yml,
            docker_service_config.name,
            docker_service_config.yml,
            docker_service_config.name
        ),
        &pb,
    );

    pb.finish_and_clear();

    info!(
        "{:?}",
        resp.into_iter().filter(|x| x.contains("Container")).last()
    );

    sleep(Duration::from_secs(5));
}
