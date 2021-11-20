//listing 25-1 (page 536)
use std::os::raw::{c_int, c_void};

use libc::{exit, atexit};

use rlpi::libc::stdlib::{on_exit};
use rlpi::error_functions::{fatal};

extern "C" fn at_exit_func1() {
    println!("atexit function 1 called");
}

extern "C" fn at_exit_func2() {
    println!("atexit function 2 called");
}

extern "C" fn on_exit_func(exit_status: c_int, arg: *const c_void) {
    let arg = unsafe { *(arg as *const u64) };
    println!("on_exit function called: status={}, arg={}", exit_status, arg);
}

pub fn main() -> ! {
    if unsafe { on_exit(on_exit_func, &10u64 as *const u64 as *const c_void) } != 0 {
        fatal("on_exit 1");
    }
    if unsafe { atexit(at_exit_func1) } != 0 {
        fatal("atexit 1");
    }
    if unsafe { atexit(at_exit_func2) } != 0 {
        fatal("atexit 2");
    }
    unsafe { exit(2); }
}
