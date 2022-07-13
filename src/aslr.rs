use crate::{die, CheckResult};

const DYNAMIC: libc::c_int = 0;
const STATIC: libc::c_int = 1;

extern "C" {
    fn detect_aslr(arg: libc::c_int) -> libc::c_int;
}

#[derive(Debug, PartialEq)]
pub enum AslrResult {
    AslrEnabled,
    Unknwon,
    AslrDisabled,
}

impl CheckResult for AslrResult {
    fn crash_if_exists(&self) {
        if self == &AslrResult::AslrDisabled {
            die();
        }
    }
}

pub fn check(is_static: bool) -> AslrResult {
    let ret = unsafe { detect_aslr(if is_static { STATIC } else { DYNAMIC }) as i32 };

    match ret {
        0 => AslrResult::AslrEnabled,
        1 => AslrResult::Unknwon,
        2 => AslrResult::AslrDisabled,
        _ => {
            panic!("unknown result");
        }
    }
}
