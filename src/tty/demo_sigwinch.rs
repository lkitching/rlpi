//listing 62-5 (page 1320)
use std::ptr;
use std::os::raw::{c_int};
use std::mem::{MaybeUninit};

use libc::{SIGWINCH, winsize, pause, sigaction, sighandler_t, ioctl, STDIN_FILENO, TIOCGWINSZ};

use rlpi::error_functions::{err_exit};
use rlpi::signals::signal_functions::{sig_empty_set};

extern "C" fn sigwinch_handler(_sig: c_int) {
}

pub fn main() {
    let sa = sigaction {
		sa_flags: 0,
		sa_sigaction: sigwinch_handler as extern "C" fn(c_int) as sighandler_t,
		sa_mask: sig_empty_set(),
		sa_restorer: None
    };

    if unsafe { sigaction(SIGWINCH, &sa, ptr::null_mut()) } == -1 {
		err_exit("sigaction");
    }

    loop {
		let ws = unsafe {
			pause();
			let mut ws: MaybeUninit<winsize> = MaybeUninit::uninit();
			if ioctl(STDIN_FILENO, TIOCGWINSZ, ws.as_mut_ptr()) == -1 {
			err_exit("ioctl");
			}

			ws.assume_init()
		};

		println!("Caught SIGWINCH, new window size: {} rows * {} columns", ws.ws_row, ws.ws_col);
    }
}
