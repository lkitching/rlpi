//listing 22-7 (page 437)
use std::ptr;
use std::os::raw::{c_void};
use std::mem::{self, MaybeUninit};

use libc::{getpid, sigaddset, sigprocmask, SIG_BLOCK, signalfd_siginfo, signalfd, read, size_t};

use crate::error_functions::{usage_err, err_exit};
use crate::signals::signal_functions::{sig_empty_set, SI_QUEUE};

pub fn main(args: &[String]) -> ! {
    if args.len() < 2 {
	usage_err(&format!("{} sig-num...", args[0]));
    }

    println!("{}: PID={}", args[0], unsafe { getpid() });

    let mut mask = sig_empty_set();
    for arg in args.iter().skip(1) {
	let sig = arg.parse().expect("Invalid number");
	unsafe { sigaddset(&mut mask, sig) };
    }

    if unsafe { sigprocmask(SIG_BLOCK, &mask, ptr::null_mut()) } == -1 {
	err_exit("sigprocmask");
    }

    let mut sfd = unsafe { signalfd(-1, &mask, 0) };
    if sfd == -1 {
	err_exit("signalfd");
    }

    loop {
	let mut fdsi = MaybeUninit::<signalfd_siginfo>::uninit();
	let s = unsafe { read(sfd, fdsi.as_mut_ptr() as *mut c_void, mem::size_of::<signalfd_siginfo>()) };
	if s == -1 {
	    err_exit("read");
	} else if s as size_t != mem::size_of::<signalfd_siginfo>() {
	    err_exit("read");
	}
	
	let fdsi = unsafe { fdsi.assume_init() };

	print!("{}: got signal {}", args[0], fdsi.ssi_signo);
	if fdsi.ssi_code == SI_QUEUE {
	    print!("; ssi_pid = {} ssi_int = {}", fdsi.ssi_pid, fdsi.ssi_int);
	}
	println!("");	
    }
}
