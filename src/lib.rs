// pub mod breakpoint;
// pub mod dummy_breakpoint;
pub mod aslr;
pub mod env;
pub mod ptrace;
pub mod vdso;

fn die() {
    panic!("found debugger");
}

pub trait CheckResult {
    fn crash_if_exists(&self);
}
