//listing 20-1 (page 399)
use std::os::raw::{c_int, c_void};

use libc::{signal, SIGINT, SIG_ERR, sleep, sighandler_t};

use rlpi::error_functions::{err_exit};

extern "C" fn sig_handler(_sig: c_int) {
    println!("Ouch!");
}

pub fn main() {
    let cb = (sig_handler as extern fn(c_int)) as *mut c_void as sighandler_t;
    if unsafe { signal(SIGINT, cb) } == SIG_ERR {
        err_exit("signal");
    }

    for i in 0.. {
        println!("{}", i);
        unsafe { sleep(3); }
    }
}
