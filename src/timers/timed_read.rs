//listing 23-2 (page 486)
use std::os::raw::{c_int, c_void};
use std::ptr;
use std::ffi::{CStr};

use libc::{exit, EXIT_SUCCESS, SA_RESTART, sigaction, sighandler_t, read, STDIN_FILENO, EINTR, SIGALRM, alarm};

use crate::libc::{errno, set_errno};
use crate::error_functions::{usage_err, err_exit, err_msg};
use crate::signals::signal_functions::{sig_empty_set};
use crate::util::{numeric_arg_or};

extern "C" fn handler(sig: c_int) {
    println!("Caught signal");
}

pub fn main(args: &[String]) -> !{
    if args.len() > 1 && args[1].as_str() == "--help" {
	usage_err(&format!("{} [num-secs [restart-flag]]", args[0]));
    }

    // setup handler for SIGALRM. Allow system calls to be interrupted unless
    // second command-line argument was supplied

    let sa = sigaction {
	sa_flags: if args.len() > 2 { SA_RESTART } else { 0 },
	sa_mask: sig_empty_set(),
	sa_sigaction: handler as extern "C" fn(c_int) as sighandler_t,
	sa_restorer: None
    };

    if unsafe { sigaction(SIGALRM, &sa, ptr::null_mut()) } == -1 {
	err_exit("sigaction");
    }

    let timeout = numeric_arg_or(&args, 1, 10);
    unsafe { alarm(timeout) };

    let mut buf: [u8; 200] = [0; 200];
    let num_read = unsafe { read(STDIN_FILENO, buf.as_mut_ptr() as *mut c_void, buf.len()) };

    let saved_errno = errno();
    unsafe { alarm(0); }	//ensure alarm is turned off
    set_errno(saved_errno);

    // determine the result of the call to read
    if num_read == -1 {
	//interrupted
	if errno() == EINTR {
	    println!("Read timed out");
	} else {
	    err_msg("read");
	}
    } else {
	let cs = CStr::from_bytes_with_nul(&buf[0..(num_read + 1) as usize]).expect("Could not create CStr");
	println!("Successful read ({} bytes): {}", num_read, cs.to_str().expect("Failed to read CStr"));
    }
    
    unsafe { exit(EXIT_SUCCESS) };
}
