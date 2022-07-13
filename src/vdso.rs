use crate::{die, CheckResult};

extern "C" {
    fn detect_vdso() -> libc::c_int;
}

#[derive(Debug, PartialEq)]
pub enum VdsoResult {
    ValidDistance,
    Unknown,
    InvalidDistance,
}

impl CheckResult for VdsoResult {
    fn crash_if_exists(&self) {
        if self == &Self::InvalidDistance {
            die();
        }
    }
}

pub fn check() -> VdsoResult {
    let ret = unsafe { detect_vdso() as i32 };

    match ret {
        0 => VdsoResult::ValidDistance,
        1 => VdsoResult::Unknown,
        2 => VdsoResult::InvalidDistance,
        _ => {
            panic!("unknown result");
        }
    }
}
