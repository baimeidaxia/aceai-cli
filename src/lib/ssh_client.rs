use log::info;
use ssh_rs::{ssh, ChannelExec, Session};

pub fn ssh(ip: &String, account: &String, password: &String, cmd: &String) {
    let mut session: Session = ssh::create_session();
    session.set_user_and_password(account, password);
    session.connect(format!("{}:22", ip)).unwrap();
    let exec: ChannelExec = session.open_exec().unwrap();
    let vec: Vec<u8> = exec.send_command(cmd).unwrap();
    info!("ssh {}\n{}", cmd, String::from_utf8(vec).unwrap());
}
