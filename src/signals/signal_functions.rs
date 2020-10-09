use std::ffi::{CStr, CString};
use std::os::raw::{c_int};
use std::mem::{MaybeUninit};
use std::ptr;

use libc::{FILE, sigset_t, strsignal, fprintf, sigismember, SIG_BLOCK, sigprocmask, sigpending};

// #define __SIGRTMIN 32          (bits/signum-generic.h)
// #define __SIGRTMAX __SIGRTMIN  (bits/signum-generic.h)
// #define _NGIG (__SIGRTMAX + 1) (bits/signum-generic.h)
// #define NSIG _NSIG             (signal.h)
const NSIG: c_int = 33;

//NOTE: The following functions use fprintf which is not async signal-safe!

fn str_signal(sig: c_int) -> String {
    let s = unsafe { strsignal(sig) };
    unsafe { CStr::from_ptr(s) }.to_str().expect("Failed to read CStr").to_owned()
}

pub fn print_sigset(of: *mut FILE, prefix: &str, sigset: *const sigset_t) {
    let mut cnt = 0;
    for sig in 1..NSIG {
	if unsafe { sigismember(sigset, sig) } != 0 {
	    cnt = cnt + 1;
	    let msg = format!("{}{} ({})\n", prefix, sig, str_signal(sig));
	    let msg_s = unsafe { CString::new(msg.as_str()).expect("Failed to create CString") };
	    unsafe { fprintf(of, msg_s.as_ptr()); }
	}
    }

    if cnt == 0 {
	let msg = format!("{}<empty signal set>", prefix);
	let msg_s = unsafe { CString::new(msg.as_str()).expect("Failed to create CString") };
	unsafe { fprintf(of, msg_s.as_ptr()); }
    }
}

pub fn print_sig_mask(of: *mut FILE, msg: &str) -> Result<(), ()> {
    let mut current_mask = unsafe { MaybeUninit::uninit() };
    if unsafe { sigprocmask(SIG_BLOCK, ptr::null(), current_mask.as_mut_ptr()) } == -1 {
	return Err(())
    }

    let current_mask = unsafe { current_mask.assume_init() };
    print_sigset(of, "\t\t", &current_mask);
    Ok(())
}

pub fn print_pending_sigs(of: *mut FILE, msg: &str) -> Result<(), ()> {
    let mut pending_sigs = unsafe { MaybeUninit::uninit() };
    if unsafe { sigpending(pending_sigs.as_mut_ptr()) } == -1 {
	return Err(())
    }

    let pending_sigs = unsafe { pending_sigs.assume_init() };
    print_sigset(of, "\t\t", &pending_sigs);
    Ok(())
}
