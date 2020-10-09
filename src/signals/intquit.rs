//listing 20-2 (page 401)
use std::os::raw::{c_void, c_int};
use std::sync::atomic::{AtomicUsize, Ordering};

use libc::{signal, SIGINT, SIGQUIT, SIG_ERR, sighandler_t, pause, exit, EXIT_SUCCESS};

use crate::error_functions::{err_exit};

static COUNT: AtomicUsize = AtomicUsize::new(0);

extern "C" fn sig_handler(sig: c_int) {
    // UNSAFE: This handler uses non-async-signal-safe functions
    // (println!(?), exit)
    if sig == SIGINT {
	let prev = COUNT.fetch_add(1, Ordering::SeqCst);
	println!("Caught SIGINT ({})", prev + 1);
	return;	//resume execution at point of interruption
    }

    //must be SIGQUIT - print a message and terminate the process
    println!("Caught SIGQUIT");
    unsafe { exit(EXIT_SUCCESS); }
}

pub fn main(args: &[String]) {
    let cb = (sig_handler as extern fn(c_int)) as *mut c_void as sighandler_t;
    if unsafe { signal(SIGINT, cb) } == SIG_ERR {
	err_exit("signal");
    }
    if unsafe { signal(SIGQUIT, cb) } == SIG_ERR {
	err_exit("signal");
    }

    loop {
	unsafe { pause(); }
    }
}
