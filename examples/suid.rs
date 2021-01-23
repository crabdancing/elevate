#[macro_use]
extern crate log;
extern crate simple_logger;

fn main() {
    simple_logger::SimpleLogger::new()
        .init()
        .expect("unable to initialize logger");

    uid_euid("①");

    spawn("/usr/bin/id");

    sudo::escalate_if_needed().expect("sudo failed");

    uid_euid("②");

    spawn("/usr/bin/id");
}

fn uid_euid(nth: &str) {
    let euid = unsafe { libc::geteuid() };
    let uid = unsafe { libc::getuid() };
    info!("{} uid: {}; euid: {};", nth, uid, euid);
}

fn spawn(cmd: &str) {
    let mut child = std::process::Command::new(cmd)
        .spawn()
        .expect("unable to start child");

    let _ecode = child.wait().expect("failed to wait on child");
}
