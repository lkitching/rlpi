//listing 23-3 (page 490)
use std::ptr;
use std::os::raw::{c_int};
use std::mem::{MaybeUninit};

use libc::{exit, EXIT_SUCCESS, timespec, sigaction, sighandler_t, SIGINT, timeval, gettimeofday, nanosleep, EINTR};

use crate::libc::{errno};
use crate::error_functions::{usage_err, err_exit};
use crate::signals::signal_functions::{sig_empty_set};

extern "C" fn sigint_handler(sig: c_int) {
    // just interrupt nanosleep()
}

pub fn main(args: &[String]) -> ! {
    if args.len() != 3 {
	usage_err(&format!("{} secs nanosecs", args[0]));
    }

    let mut request = timespec {
	tv_sec: args[1].parse().expect("Invalid seconds"),
	tv_nsec: args[2].parse().expect("Invalid nanoseconds")
    };

    // allow SIGINT handler to interrupt nanosleep()
    let sa = sigaction {
	sa_flags: 0,
	sa_sigaction: sigint_handler as extern "C" fn(c_int) as sighandler_t,
	sa_mask: sig_empty_set(),
	sa_restorer: None
    };

    if unsafe { sigaction(SIGINT, &sa, ptr::null_mut()) } == -1 {
	err_exit("sigaction");
    }

    let mut start: MaybeUninit::<timeval> = MaybeUninit::uninit();
    if unsafe { gettimeofday(start.as_mut_ptr(), ptr::null_mut()) } == -1 {
	err_exit("gettimeofday");
    }
    let start = unsafe { start.assume_init() };

    loop {
	let mut remain: MaybeUninit<timespec> = MaybeUninit::uninit();
	let s = unsafe { nanosleep(&request, remain.as_mut_ptr()) };
	if s == -1 && errno() != EINTR {
	    err_exit("nanosleep");
	}

	let remain = unsafe { remain.assume_init() };

	let mut finish: MaybeUninit::<timeval> = MaybeUninit::uninit();
	if unsafe { gettimeofday(finish.as_mut_ptr(), ptr::null_mut()) } == -1 {
	    err_exit("gettimeofday");
	}
	let finish = unsafe { finish.assume_init() };

	let period = (finish.tv_sec - start.tv_sec) as f64 + (finish.tv_usec + start.tv_usec) as f64 / 1000000.0;
	println!("Slept for: {} secs", period);

	if s == 0 {
	    // sleep completed
	    break;
	} else {
	    println!("Remaining: {}.{}", remain.tv_sec, remain.tv_nsec);
	    request = remain;
	}
    }

    println!("Sleep complete");
    unsafe { exit(EXIT_SUCCESS) };
}
