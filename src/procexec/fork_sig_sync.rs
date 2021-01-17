// listing 24-6 (page 528)
use std::ptr;
use std::os::raw::{c_int};
use std::thread;
use std::time::{Duration};
use std::mem::{MaybeUninit};

use libc::{exit, _exit, EXIT_SUCCESS, setbuf, sigaddset, SIGUSR1, sigaction, sighandler_t, SA_RESTART,
           fork, getpid, getppid, kill, EINTR, SIG_BLOCK, sigset_t, sigprocmask, sigsuspend, SIG_SETMASK};

use crate::libc::{errno};
use crate::libc::stdio::{stdout};
use crate::curr_time::{curr_time};
use crate::error_functions::{err_exit};
use crate::signals::signal_functions::{sig_empty_set};

const SYNC_SIG: c_int = SIGUSR1;

extern "C" fn handler(sig: c_int) {
}

pub fn main(args: &[String]) -> ! {
    //disable buffering of stdout
    unsafe { setbuf(stdout, ptr::null_mut()) };

    let mut block_mask = sig_empty_set();
    unsafe { sigaddset(&mut block_mask, SYNC_SIG) };
    let mut orig_mask: MaybeUninit::<sigset_t> = MaybeUninit::uninit();
    if unsafe { sigprocmask(SIG_BLOCK, &block_mask, orig_mask.as_mut_ptr()) } == -1 {
	err_exit("sigprocmask");
    }
    let orig_mask = unsafe { orig_mask.assume_init() };

    let sa = sigaction {
	sa_mask: sig_empty_set(),
	sa_flags: SA_RESTART,
	sa_sigaction: handler as extern "C" fn(c_int) as sighandler_t,
	sa_restorer: None
    };

    if unsafe { sigaction(SYNC_SIG, &sa, ptr::null_mut()) } == -1 {
	err_exit("sigaction");
    }

    let child_pid = unsafe { fork() };
    match child_pid {
	-1 => {
	    err_exit("fork()");
	},
	0 => {
	    println!("[{} {}] Child started - doing some work",
		     curr_time("%T"),
		     unsafe { getpid() });

	    // simulate time spent doing work
	    thread::sleep(Duration::from_secs(2));

	    // signal parent
	    println!("[{} {}] Child about to signal parent",
		     curr_time("%T"),
		     unsafe { getpid() });
	    let parent_pid = unsafe { getppid() };
	    if unsafe { kill(parent_pid, SYNC_SIG) } == -1 {
		err_exit("kill");
	    }

	    // continue child execution ...
	    unsafe { _exit(EXIT_SUCCESS) };
	},
	_ => {
	    // parent
	    println!("[{} {}] Parent about to wait for signal",
		     curr_time("%T"),
		     unsafe { getpid() });

	    let empty_mask = sig_empty_set();
	    if unsafe { sigsuspend(&empty_mask) } == -1 && errno() != EINTR {
		err_exit("sigsuspend");
	    }
	    println!("[{} {}] Parent got signal",
		     curr_time("%T"),
		     unsafe { getpid() });

	    // return signal mask to its original state
	    if unsafe { sigprocmask(SIG_SETMASK, &orig_mask, ptr::null_mut()) } == -1 {
		err_exit("sigprocmask");
	    }

	    // parent continues
	    unsafe { exit(EXIT_SUCCESS) };
	}
    }
}
