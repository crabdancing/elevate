#![deny(unsafe_code)]

use std::error::Error;

#[macro_use]
extern crate log;

#[derive(Debug, PartialEq)]
pub enum RunningAs {
    /// Root or Administrator
    Root,
    User,
}
use RunningAs::*;

#[cfg(unix)]
/// We relie on the $UID or /proc/self/status
pub fn check() -> Result<RunningAs, Box<dyn Error>> {
    let uid: String = match std::env::var("UID") {
        Ok(s) => s,
        Err(_) => {
            const NO_UID: &str = "/proc/self/status does not contain uid";
            let status = std::fs::read_to_string("/proc/self/status")?;
            let lines = status.lines();
            let uid_line = lines.filter(|&l| l.to_lowercase().starts_with("uid")).next().expect(NO_UID);
            //trace!("uid_line: {}", uid_line);
            uid_line.split_whitespace().skip(1).next().expect(NO_UID).to_string()
        }
    };

    Ok(if uid.parse::<usize>()? == 0 { Root } else { User })
}

#[cfg(unix)]
pub fn escalate_if_needed() -> Result<(), Box<dyn Error>> {
    let current = check()?;
    if current == Root {
        trace!("already running as Root");
        return Ok(());
    }
    debug!("Escalating privileges");
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
        assert!(c.is_ok(), "{:?}", c);
    }
}
