//listing 34-7 (page 728)
use std::env;
use std::ptr;
use std::os::raw::{c_int};

use libc::{getpid, getppid, getpgrp, getsid, setbuf, sigaction, sighandler_t, SIGHUP, SIGCONT, SIGSTOP, raise, pause, exit, EXIT_SUCCESS, alarm, sleep};

extern crate rlpi;
use rlpi::error_functions::{usage_err, err_exit};
use rlpi::signals::signal_functions::{sig_empty_set, str_signal};
use rlpi::libc::stdio::{stdout};
use rlpi::util::{fork_or_die, ForkResult};

extern "C" fn handler(sig: c_int) {
    println!("PID={}: caught signal {} ({})",
	     unsafe { getpid() },
	     sig,
	     str_signal(sig));
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "--help" {
	usage_err(&format!("{} {{s|p}} ...", args[0]));
    }

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

    if unsafe { sigaction(SIGCONT, &sa, ptr::null_mut()) } == -1 {
	err_exit("sigaction");
    }

    println!("parent: PID={}, PPID={}, PGID={}, SIG={}",
	     unsafe { getpid() },
	     unsafe { getppid() },
	     unsafe { getpgrp() },
	     unsafe { getsid(0) });

    // create one child for each command-line argument
    for arg in args[1..].iter() {
	match fork_or_die() {
	    ForkResult::Child => {
		println!("child: PID={}, PPID={}, PGID={}, SID={}",
			 unsafe { getpid() },
			 unsafe { getppid() },
			 unsafe { getpgrp() },
			 unsafe { getsid(0) });

		if let Some('s') = arg.chars().next() {
		    // stop via signal
		    println!("PID={} stopping", unsafe { getpid() });
		    unsafe { raise(SIGSTOP); }
		} else {
		    // ensure we die if SIGHUP not received
		    unsafe { alarm(60); }
		    
		    // wait for signal
		    println!("PID={} pausing", unsafe { getpid() });
		    unsafe { pause(); }
		}
	    },
	    ForkResult::Parent(_) => {
		// continue looping
	    }
	}
    }

    // parent continues after creating all children

    // give children a chance to start
    unsafe { sleep(3); }
    println!("Parent exiting");
    unsafe { exit(EXIT_SUCCESS); }
}
