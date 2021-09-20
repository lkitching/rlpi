//listing 21-1 (page 424)
use std::os::raw::{c_char, c_int, c_void};
use std::ffi::{CString};
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};

use libc::{strdup, sigaction, sighandler_t, SIGINT, strcmp};

use crate::error_functions::{usage_err, err_exit};
use super::signal_functions::{sig_empty_set};
use crate::libc::unistd::{crypt};

static SALT_BUF: [c_char; 3] = [120,120,0];
static STR2_BUF: AtomicUsize = AtomicUsize::new(0);
static HANDLED: AtomicUsize = AtomicUsize::new(0);

extern "C" fn handler(_sig: c_int) {
    let p = STR2_BUF.load(Ordering::SeqCst) as *const c_char;
    unsafe { crypt(p, SALT_BUF.as_ptr()); }
    HANDLED.fetch_add(1, Ordering::SeqCst);
}

pub fn main(args: &[String]) {
    if args.len() != 3 {
	usage_err(&format!("{} str1 str2\n", args[0]));
    }

    let str1_s = CString::new(args[1].as_str()).expect("Failed to create CString");
    let buf = unsafe { crypt(str1_s.as_ptr(), SALT_BUF.as_ptr()) };
    let cr1 = unsafe { strdup(buf) };

    let str2_s = CString::new(args[2].as_str()).expect("Failed to create CString");
    STR2_BUF.store(str2_s.as_ptr() as usize, Ordering::SeqCst);

    if cr1.is_null() {
	err_exit("strdup");
    }

    let sa = sigaction {
	sa_sigaction: handler as extern fn(c_int) as *const c_void as sighandler_t,
	sa_mask: sig_empty_set(),
	sa_flags: 0,
	sa_restorer: None
    };

    if unsafe { sigaction(SIGINT, &sa, ptr::null_mut()) } == -1 {
	err_exit("sigaction");
    }

    // repeatedly call crypt() using args[1]. If interrupted by a
    // signal handler, the static storage returned by crypt() will be
    // overwritten by the results of encrypting args[2] and strcmp
    // will detect a mismatch with the value in cr1
    let handled = 0;
    let mut mismatches = 0;
    for call_num in 1.. {
	let enc_buf = unsafe { crypt(str1_s.as_ptr(), SALT_BUF.as_ptr()) };
	if unsafe { strcmp(enc_buf, cr1) != 0 } {
	    mismatches = mismatches + 1;
	    println!("Mismatch on call {} (mismatch={} handled={})", call_num, mismatches, handled);
	}
    }    
}
