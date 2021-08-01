//listing 34-6 (page 724)
use std::env;
use std::ptr;
use std::os::raw::{c_int};
use std::mem::{MaybeUninit};

use libc::{SIGTSTP, SIG_DFL, SIG_ERR, raise, sigaddset, sigprocmask, sigset_t, sigaction, sighandler_t, SA_RESTART, pause, signal, SIG_UNBLOCK, SIG_IGN};

extern crate rlpi;

use rlpi::libc::{errno, set_errno};
use rlpi::error_functions::{err_exit};
use rlpi::signals::signal_functions::{sig_empty_set};

extern "C" fn tstp_handler(sig: c_int) {
    let saved_errno = errno();

    println!("Caught SIGTSTP");

    // set handling to default
    if unsafe { signal(SIGTSTP, SIG_DFL) } == SIG_ERR {
	err_exit("signal");
    }

    // generate further SIGTSTP
    unsafe { raise(SIGTSTP); }

    // unblock SIGTSTP - the pending SIGTSTP immediately suspends the program
    let mut tstp_mask = sig_empty_set();
    unsafe { sigaddset(&mut tstp_mask, SIGTSTP); }

    let prev_mask = unsafe {
	let mut prev_mask: MaybeUninit<sigset_t> = MaybeUninit::uninit();
	if sigprocmask(SIG_UNBLOCK, &tstp_mask, prev_mask.as_mut_ptr()) == -1 {
	    err_exit("sigprocmask");
	}

	prev_mask.assume_init()
    };

    // execution resumes here after SIGCONT

    // re-block SIGTSTP
    let sa = sigaction {
	sa_mask: sig_empty_set(),
	sa_flags: SA_RESTART,
	sa_sigaction: tstp_handler as extern "C" fn(c_int) as sighandler_t,
	sa_restorer: None
    };

    if unsafe { sigaction(SIGTSTP, &sa, ptr::null_mut()) } == -1 {
	err_exit("sigaction");
    }
    

    println!("Exiting SIGTSTP handler");
    set_errno(saved_errno);
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    // only establish handler for SIGTSTP if it's not being ignored
    let existing_sa = unsafe {
	let mut sa: MaybeUninit<sigaction> = MaybeUninit::uninit();
	if sigaction(SIGTSTP, ptr::null(), sa.as_mut_ptr()) == -1 {
	    err_exit("sigaction");
	}
	sa.assume_init()
    };

    if existing_sa.sa_sigaction != SIG_IGN {
	let sa = sigaction {
	    sa_mask: sig_empty_set(),
	    sa_flags: SA_RESTART,
	    sa_sigaction: tstp_handler as extern "C" fn(c_int) as sighandler_t,
	    sa_restorer: None
	};

	if unsafe { sigaction(SIGTSTP, &sa, ptr::null_mut()) } == -1 {
	    err_exit("sigaction");
	}
    }

    // wait for signals
    loop {
	unsafe { pause(); }
	println!("Main");
    }
	
}
