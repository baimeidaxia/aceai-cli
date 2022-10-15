use clap::{arg, Command};

pub fn init() -> Command {
    clap::Command::new("aceai-dp-cli")
        .bin_name("aceai-dp-cli")
        .version("1.0")
        .author("baimeidaxia")
        .subcommand_required(true)
        .subcommand(
            Command::new("dcs")
                .about("deploy nacos service")
                .arg(arg!(<service_name> "please input the micro service name in nacos"))
                .arg_required_else_help(true),
        )
}
