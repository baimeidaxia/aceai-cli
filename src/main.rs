mod lib;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let cmd = lib::cmd::init();
    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("dcs", sub_matches)) => lib::cmd_deploy::handle(&sub_matches),
        _ => Ok(()),
    }
}
