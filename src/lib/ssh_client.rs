use log::info;
use ssh_rs::{ssh, ChannelExec, Session};

use super::conf::ConfigServer;

pub fn ssh(ip: &String, account: &String, password: &String, cmd: &String) -> Vec<u8> {
    let mut session: Session = ssh::create_session();
    session.set_user_and_password(account, password);
    session.connect(format!("{}:22", ip)).unwrap();
    let exec: ChannelExec = session.open_exec().unwrap();
    let vec: Vec<u8> = exec.send_command(&cmd).unwrap();
    vec
}

pub fn ssh_by_config(server: &ConfigServer, cmd: String) -> Vec<u8> {
    ssh(&server.ip, &server.account, &server.password, &cmd)
}

pub fn findDockerComposeFiles(server: &ConfigServer) {
    let res = ssh_by_config(
        server,
        format!("cd {}; ls *.yml", server.docker_compose_path),
    );
    info!("find yml {}", String::from_utf8(res).unwrap());
    //    for ele in res {
    //        let content = ssh_by_config(
    //            server,
    //            format!("cat {}/{}", server.docker_compose_path, ele),
    //        );
    //        info!("content {:?}", ele);
    //    }
}
