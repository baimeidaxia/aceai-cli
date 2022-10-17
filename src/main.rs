use lib::conf;

mod lib;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let _ = conf::load()?;
    let cmd = lib::cmd::init();
    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("dms", sub_matches)) => lib::cmd_deploy_service::handle(&sub_matches),
        Some(("ssh", sub_matches)) => lib::cmd_ssh::handle(&sub_matches),
        _ => Ok(()),
    }
}
