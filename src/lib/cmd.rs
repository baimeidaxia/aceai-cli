use clap::{arg, Command};

pub fn init() -> Command {
    clap::Command::new("aceai-dp-cli")
        .bin_name("aceai-dp-cli")
        .version("1.0")
        .author("baimeidaxia")
        .subcommand_required(true)
        .subcommand(
            Command::new("dms")
                .about("deploy micro service")
                .arg(arg!(<service_name> "please input the micro service name in nacos"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("dw")
                .about("deploy website")
                .arg(arg!(<website_name> "please input the website name"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("ssh")
                .about("execute ssh command")
                .arg(arg!(<ip> "please input the server ip"))
                .arg(arg!(<account> "please input the server acount"))
                .arg(arg!(<password> "please input the server password"))
                .arg(arg!(<cmd> "please input the cmd to execute"))
                .arg_required_else_help(true),
        )
}
