use std::error::Error;
use std::process::Command;

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
    let mut command: Command = Command::new("/usr/bin/sudo");

    if let Ok(trace) = std::env::var("RUST_BACKTRACE") {
        let value = match &*trace {
            "" => None,
            "1" | "true" | "TRUE" => Some("1"),
            "full" => Some("full"),
            invalid => {
                warn!(
                    "RUST_BACKTRACE has invalid value {:?} -> defaulting to \"full\"",
                    invalid
                );
                Some("full")
            }
        };
        if let Some(value) = value {
            trace!("relaying RUST_BACKTRACE={}", value);
            command.arg(format!("RUST_BACKTRACE={}", value));
        }
    }

    let mut child = command.args(args).spawn().expect("failed to execute child");

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
