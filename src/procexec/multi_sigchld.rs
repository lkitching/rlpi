//listing 26-5 (page 557)
use std::{ptr, thread, time};
use std::os::raw::{c_int};
use std::sync::atomic::{AtomicUsize, Ordering};

use libc::{setbuf, sigaction, sighandler_t, sigaddset, fork, SIGCHLD, SIG_SETMASK, getpid, _exit, EXIT_SUCCESS, exit, EINTR,
           sigsuspend, sigprocmask, waitpid, WNOHANG, ECHILD};

use crate::libc::{errno, set_errno};
use crate::libc::stdio::{stdout};
use crate::error_functions::{usage_err, err_exit, err_msg};
use crate::signals::signal_functions::{sig_empty_set};
use crate::curr_time::{curr_time};
use super::print_wait_status::{print_wait_status};

static NUM_LIVE_CHILDREN: AtomicUsize = AtomicUsize::new(0);

extern "C" fn handler(_sig: c_int) {
    // WARNING: This handler uses non-async-safe functions
    let saved_errno = errno();

    println!("{} handler: Caught SIGCHLD", curr_time("%T"));

    let mut status = -1;
    let mut child_pid = unsafe { waitpid(-1, &mut status, WNOHANG) };
    while status > 0 {
	println!("{} handler: Reaped child {}",
		 curr_time("%T"),
		 child_pid);
	print_wait_status(None, status);

	NUM_LIVE_CHILDREN.fetch_sub(1, Ordering::SeqCst);
	
	child_pid = unsafe { waitpid(-1, &mut status, WNOHANG) };
    }

    if child_pid == -1 && errno() != ECHILD {
	err_msg("waitpid");
    }

    // artificially lengthen execution of handler
    thread::sleep(time::Duration::from_secs(5));    
    println!("{} handler: returning", curr_time("%T"));

    set_errno(saved_errno);
}

pub fn main(args: &[String]) -> ! {
    if args.len() < 2 || args.len() > 1 && &args[1] == "--help" {
	usage_err(&format!("{} child-sleep-time ...", &args[0]));
    }

    unsafe { setbuf(stdout, ptr::null_mut()); }
    NUM_LIVE_CHILDREN.store(args.len() - 1, Ordering::SeqCst);

    let sa = sigaction {
	sa_flags: 0,
	sa_mask: sig_empty_set(),
	sa_sigaction: handler as extern "C" fn(c_int) as sighandler_t,
	sa_restorer: None
    };

    if unsafe { sigaction(SIGCHLD, &sa, ptr::null_mut()) } == -1 {
	err_exit("sigaction");
    }

    // block SIGCHLD to prevent its delivery if a child terminates
    // before the parent commences the sigsuspend() loop below
    let mut block_mask = sig_empty_set();
    unsafe { sigaddset(&mut block_mask, SIGCHLD); }
    if unsafe { sigprocmask(SIG_SETMASK, &block_mask, ptr::null_mut()) } == -1 {	
	err_exit("sigprocmask");
    }

    for j in 1..args.len() {
	let sleep_str = &args[j];
	let sleep_seconds = sleep_str.parse().expect("Invalid sleep period");
	match unsafe { fork() } {
	    -1 => {
		err_exit("fork");
	    },
	    0 => {
		// child - sleep then exit
		thread::sleep(time::Duration::from_secs(sleep_seconds));
		println!("{} Child {} (PID={}) exiting",
			 curr_time("%T"),
			 j,
			 unsafe { getpid() });
		unsafe { _exit(EXIT_SUCCESS); }
	    },
	    _ => {
		// parent - loop to create next child
	    }
	}
    }

    // parent - wait for SIGCHLD until all children are dead
    let empty_mask = sig_empty_set();
    let mut sig_cnt = 0;

    while NUM_LIVE_CHILDREN.load(Ordering::SeqCst) > 0 {
	if unsafe { sigsuspend(&empty_mask) == -1 && errno() != EINTR } {
	    err_exit("sigsuspend");
	}
	sig_cnt = sig_cnt + 1;
    }

    println!("{} All {} children have terminated; SIGCHLD was caught {} times",
	     curr_time("%T"),
	     args.len() - 1,
	     sig_cnt);
    unsafe { exit(EXIT_SUCCESS) }
}
