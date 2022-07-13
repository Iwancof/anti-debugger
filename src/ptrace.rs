use nix::errno::Errno;
use nix::sys::ptrace::{detach, traceme};
use nix::unistd::Pid;

use crate::{die, CheckResult};

#[derive(Debug)]
pub enum PtraceResult {
    NotFound,
    Found(Errno),
}

impl CheckResult for PtraceResult {
    fn crash_if_exists(&self) {
        match self {
            Self::Found(_) => {
                die();
            }
            _ => {}
        }
    }
}

pub fn check() -> PtraceResult {
    if let Err(e) = traceme() {
        return PtraceResult::Found(e);
    }

    // let _ = detach(Pid::from_raw(0), None);

    PtraceResult::NotFound
}
