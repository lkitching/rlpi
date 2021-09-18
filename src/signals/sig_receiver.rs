//listing 20-7 (page 414)
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::os::raw::{c_int, c_void};
use std::ptr;
use std::mem::{MaybeUninit};

use libc::{exit, EXIT_SUCCESS, getpid, signal, sighandler_t, sigfillset, sigprocmask, sleep, SIG_SETMASK,
           sigpending, sigemptyset, SIGINT};

use super::signal_functions::{print_sigset};
use crate::error_functions::{err_exit};
use crate::libc::stdio::{stdout};
use crate::libc::signal::{NSIG};

static SIGNAL_COUNTS: [AtomicUsize; NSIG as usize] = [
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0)];

static GOT_SIGINT: AtomicBool = AtomicBool::new(false);

extern "C" fn sig_handler(sig: c_int) {
    if sig == SIGINT {
	GOT_SIGINT.store(true, Ordering::SeqCst);
    } else {
	SIGNAL_COUNTS[sig as usize].fetch_add(1, Ordering::SeqCst);
    }
}

pub fn main(args: &[String]) -> ! {
    let pid = unsafe { getpid() };
    println!("{}: PID is {}", args[0], pid);

    //use same handler for all signals
    let cb = sig_handler as extern fn(c_int) as *mut c_void as sighandler_t;
    for sig in 1..NSIG {
	unsafe { signal(sig, cb); }	//ignore errors
    }

    // if a sleep time was specified, temporarily block all signals,
    // sleep (while another process sends us signals), and then
    // display the mask of pending signals and unblock all signals
    if args.len() > 1 {
	let num_secs = args[1].parse().expect("Invalid sleep period");

	let mut blocking_mask = MaybeUninit::uninit();
	unsafe { sigfillset(blocking_mask.as_mut_ptr()); }
	let blocking_mask = unsafe { blocking_mask.assume_init() };

	if unsafe { sigprocmask(SIG_SETMASK, &blocking_mask, ptr::null_mut()) } == -1 {
	    err_exit("sigprocmask");
	}

	println!("{}: sleeping for {} seconds", args[0], num_secs);
	unsafe { sleep(num_secs); }

	//get pending signals
	let mut pending_mask = MaybeUninit::uninit();
	if unsafe { sigpending(pending_mask.as_mut_ptr()) } == -1 {
	    err_exit("sigpending");
	}
	let pending_mask = unsafe { pending_mask.assume_init() };

	println!("{}: pending signals are: ", args[0]);
	print_sigset(unsafe { stdout }, "\t\t", &pending_mask);

	//unblock all signals
	let mut empty_mask = MaybeUninit::uninit();
	if unsafe { sigemptyset(empty_mask.as_mut_ptr()) } == -1 {
	    err_exit("sigprocmask");
	}
	let empty_mask = unsafe { empty_mask.assume_init() };

	if unsafe { sigprocmask(SIG_SETMASK, &empty_mask, ptr::null_mut()) } == -1 {
	    err_exit("sigprocmask");
	}

	//wait until SIGINT caught
	loop {
	    if GOT_SIGINT.load(Ordering::SeqCst) {
		break;
	    }
	}

	//display number of signals received
	for n in 1..(NSIG as usize) {
	    let count = SIGNAL_COUNTS[n].load(Ordering::SeqCst);
	    if count != 0 {
		println!("{}: signal {} caught {} times", args[0], n, count);
	    }
	}
    }
    
    unsafe { exit(EXIT_SUCCESS); }
}
