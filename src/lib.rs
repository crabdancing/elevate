use std::error::Error;

#[macro_use]
extern crate log;

#[derive(Debug, PartialEq)]
pub enum RunningAs {
    /// Root or Administrator
    Root,
    /// Running as a normal user
    User,
    /// Started from SUID
    Suid,
}
use RunningAs::*;

#[cfg(unix)]
/// Check geteuid() to see if we match uid == 0
pub fn check() -> RunningAs {
    let uid = unsafe { libc::getuid() };
    let euid = unsafe { libc::geteuid() };

    match (uid, euid) {
        (0, 0) => Root,
        (_, 0) => Suid,
        (_, _) => User,
    }
    //if uid == 0 { Root } else { User }
}

#[cfg(unix)]
pub fn escalate_if_needed() -> Result<(), Box<dyn Error>> {
    let current = check();
    trace!("Running as {:?}", current);
    match current {
        Root => {
            trace!("already running as Root");
            return Ok(());
        }
        User => {
            debug!("Escalating privileges");
        }
        Suid => {
            trace!("setuid(0)");
            unsafe {
                libc::setuid(0);
            }
            return Ok(());
        }
    }
    let args = std::env::args();
    let mut child = std::process::Command::new("/usr/bin/sudo")
        .args(args)
        .spawn()
        .expect("failed to execute child");

    let ecode = child.wait().expect("failed to wait on child");

    if ecode.success() == false {
        std::process::exit(ecode.code().unwrap_or(1));
    } else {
        std::process::exit(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let c = check();
        assert!(true, "{:?}", c);
    }
}
