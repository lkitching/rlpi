//listing 34-3 (page 710)
use std::ptr;
use std::os::raw::{c_int};

use libc::{setbuf, sigaction, sighandler_t, fork, setpgid, getpid, getppid, getpgrp, getsid, alarm, SIGHUP, pause};

use crate::libc::stdio::{stdout};
use crate::error_functions::{err_exit};
use crate::signals::signal_functions::{sig_empty_set};

extern "C" fn handler(sig: c_int) {
}

pub fn main(args: &[String]) {
    // disable stdout buffering
    unsafe { setbuf(stdout, ptr::null_mut()); }

    let sa = sigaction {
	sa_mask: sig_empty_set(),
	sa_flags: 0,
	sa_sigaction: handler as extern "C" fn(c_int) as sighandler_t,
	sa_restorer: None
    };

    if unsafe { sigaction(SIGHUP, &sa, ptr::null_mut()) } == -1 {
	err_exit("sigaction");
    }

    let child_pid = unsafe { fork() };
    if child_pid == -1 {
	err_exit("fork");
    }

    if child_pid == 0 && args.len() > 1 {
	// move to new process group
	if unsafe { setpgid(0, 0) } == -1 {
	    err_exit("setpgid");
	}
    }

    println!("PID={}; PPID={}; PGID={}; SID={}",
	     unsafe { getpid() },
	     unsafe { getppid() },
	     unsafe { getpgrp() },
	     unsafe { getsid(0) });

    // an unhandled SIGALRM ensures this process will die if nothing
    // else terminates it
    unsafe { alarm(60); }

    loop {
	unsafe { pause(); }
	println!("{}: caught SIGHUP", unsafe { getpid() });
    }
}
