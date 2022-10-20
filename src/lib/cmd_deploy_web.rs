use clap::ArgMatches;
use indicatif::ProgressBar;
use log::info;

use crate::lib::ssh_client;

use super::{
    conf::{self, ConfigServer},
    docker_compose_parser::DockerComposeService,
};

pub fn handle(sub_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let website_name = sub_matches.get_one::<String>("website_name").unwrap();
    info!("website {} deploying", website_name);
    let config = conf::load_global_config();
    let docker_service_configs = conf::load_docker_service_configs();
    for ele in docker_service_configs {
        // shuangma use port in 8000..9000 for website
        if ele.name.contains(website_name) && ele.port > 8000 && ele.port < 9000 {
            info!("website {} {}:{} staring", website_name, ele.ip, ele.port,);
            let server = config.get_server_by(&ele.ip);
            deploy_docker(&ele, &server);
        }
    }
    info!("website {} deploy finish", website_name);
    Ok(())
}

fn deploy_docker(docker_service_config: &DockerComposeService, server: &ConfigServer) {
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
}
