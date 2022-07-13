use crate::{die, CheckResult};
use std::env;

#[derive(Debug, PartialEq)]
pub enum EnvResult {
    GdbEnvFound,
    GdbEnvNotFound,
}

impl CheckResult for EnvResult {
    fn crash_if_exists(&self) {
        if self == &Self::GdbEnvFound {
            die();
        }
    }
}

pub fn check() -> EnvResult {
    if env::var("LINES").is_ok() {
        EnvResult::GdbEnvFound
    } else if env::var("COLUMNS").is_ok() {
        EnvResult::GdbEnvFound
    } else {
        EnvResult::GdbEnvNotFound
    }
}
