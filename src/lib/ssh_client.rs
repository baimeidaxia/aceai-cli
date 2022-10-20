use std::{thread::sleep, time::Duration};

use super::conf::ConfigServer;
use indicatif::ProgressBar;
use log::{debug, info};
use ssh_rs::{ssh, Channel, ChannelExec, ChannelShell, Session};

pub fn ssh(ip: &String, account: &String, password: &String, cmd: &String) -> Vec<u8> {
    let mut session: Session = ssh::create_session();
    session.set_user_and_password(account, password);
    session.connect(format!("{}:22", ip)).unwrap();
    let exec: ChannelExec = session.open_exec().unwrap();
    let vec: Vec<u8> = exec.send_command(&cmd).unwrap();
    vec
}

pub fn ssh_shell(
    ip: &String,
    account: &String,
    password: &String,
    cmd: &String,
    pb: &ProgressBar,
) -> Vec<String> {
    let mut session: Session = ssh::create_session();
    session.set_user_and_password(account, password);
    session.connect(format!("{}:22", ip)).unwrap();
    let channel: Channel = session.open_channel().unwrap();
    let mut shell = channel.open_shell().unwrap();
    let result = run_shell(&mut shell, cmd, pb);
    // Close channel.
    shell.close().unwrap();
    // Close session.
    session.close().unwrap();
    result
}

pub fn ssh_by_config(server: &ConfigServer, cmd: String) -> Vec<u8> {
    ssh(&server.ip, &server.account, &server.password, &cmd)
}

pub fn find_docker_compose_files(server: &ConfigServer) -> Vec<String> {
    let files = ssh_by_config(
        server,
        format!("cd {}; ls *.yml", server.docker_compose_path),
    );
    let mut result = Vec::new();
    String::from_utf8(files)
        .unwrap()
        .lines()
        .for_each(|x| result.push(x.to_string()));
    result
}

pub fn fetch_docker_compose_content(server: &ConfigServer, file: &String) -> String {
    let cat_res = ssh_by_config(
        server,
        format!("cat {}/{}", server.docker_compose_path, file),
    );
    String::from_utf8(cat_res).unwrap()
}

fn run_shell(shell: &mut ChannelShell, cmd: &String, pb: &ProgressBar) -> Vec<String> {
    let mut cmd_str = String::from(cmd);
    cmd_str.push_str("\n");
    shell.write(cmd_str.as_bytes()).unwrap();

    let mut result = Vec::new();
    let mut i = 1;
    while i < 100 {
        let vec = shell.read().unwrap();
        let len = vec.len() as i32;
        if len == 0 {
            break;
        }

        let line = String::from_utf8(vec).unwrap();
        for ele in line.lines() {
            result.push(ele.to_string());
        }
        pb.inc(1);
        pb.set_message(format!("running {} s", i));
        sleep(Duration::from_millis(1000));
        i = i + 1;
    }
    result
}
