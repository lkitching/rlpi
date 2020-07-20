use std::env;
use std::os::raw::{c_int};

#[link(name = "c")]
extern {
    fn abort() -> !;
    fn exit(status: c_int) -> !;
    fn _exit(status: c_int) -> !;
}

//see https://www.gnu.org/software/libc/manual/html_node/Exit-Status.html
enum ExitStatus {
    Success = 0,
    Failure = 1
}

fn should_dump_core() -> bool {
    match env::var("EF_DUMPCORE") {
	Ok(s) => { s.len() > 0 }
	_ => { false }
    }
}

pub fn terminate(use_exit3: bool) -> ! {
    if should_dump_core() {
	unsafe { abort() }
    } else if use_exit3 {
	unsafe { exit(ExitStatus::Failure as c_int) }
    } else {
	unsafe { _exit(ExitStatus::Failure as c_int) }
    }
}
