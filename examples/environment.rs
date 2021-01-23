#[macro_use]
extern crate log;
extern crate simple_logger;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::SimpleLogger::new()
        .init()
        .expect("unable to initialize logger");

    uid_euid("①");

    spawn("export");

    sudo::with_env(&["EXAMPLE_", "CARGO"])?;

    uid_euid("②");

    spawn("export");

    Ok(())
}

fn uid_euid(nth: &str) {
    let euid = unsafe { libc::geteuid() };
    let uid = unsafe { libc::getuid() };
    info!("{} uid: {}; euid: {};", nth, uid, euid);
}

fn spawn(cmd: &str) {
    let mut child = std::process::Command::new("/usr/bin/env")
        .args(&["bash", "-c", cmd])
        .spawn()
        .expect("unable to start child");

    let _ecode = child.wait().expect("failed to wait on child");

    println!("\n\n\n\n\n\n");
}
