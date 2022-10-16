use clap::ArgMatches;
use log::info;

use crate::lib::ssh_client;

pub fn handle(sub_matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let ip = sub_matches.get_one::<String>("ip").unwrap();
    let account = sub_matches.get_one::<String>("account").unwrap();
    let password = sub_matches.get_one::<String>("password").unwrap();
    let cmd = sub_matches.get_one::<String>("cmd").unwrap();
    info!("ssh config {} {} {}", ip, account, password);
    ssh_client::ssh(ip, account, password, cmd);
    Ok(())
}
