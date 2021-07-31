//listing 34-2
use std::ffi::{CString};

use libc::{_exit, exit, EXIT_SUCCESS, fork, setsid, getpid, getpgrp, getsid, O_RDWR, open};

use crate::error_functions::{err_exit};

pub fn main(args: &[String]) {
    let child_pid = unsafe { fork() };
    if child_pid != 0 {
	// exit if parent or on error
	unsafe { _exit(EXIT_SUCCESS); }
    }

    if unsafe { setsid() } == -1 {
	err_exit("setsid");
    }

    println!("PID={}, PGID={}, SID={}",
	     unsafe { getpid() },
	     unsafe { getpgrp() },
	     unsafe { getsid(0) });

    // NOTE: should fail
    //process has no controlling terminal after creating new session
    unsafe {
	let cs = CString::new("/dev/tty").expect("Failed to create CString");
	if unsafe { open(cs.as_ptr(), O_RDWR) } == -1 {
	    err_exit("open /dev/tty");
	}
	exit(EXIT_SUCCESS);
    };
    
}
