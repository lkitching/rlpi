//listing 24-1 (page 517)
use std::sync::atomic::{AtomicU8, Ordering};

use libc::{exit, EXIT_SUCCESS, fork, sleep, getpid};

use crate::error_functions::{err_exit};

//TODO: find out where this is allocated
const IDATA: AtomicU8 = AtomicU8::new(111);

pub fn main(args: &[String]) -> ! {
    // stack allocated
    let mut istack = 222;
    
    let child_pid = unsafe { fork() };
    
    match child_pid {
	-1 => {
	    err_exit("fork");
	},
	0 => {
	    // child process
	    istack = 3;
	    IDATA.store(3, Ordering::SeqCst);
	    println!("{}", IDATA.load(Ordering::SeqCst));
	}
	_ => {
	    // parent process
	    // give child a chance to execute
	    unsafe { sleep(3); }
	}
    }

    // both child and parent reach here
    let who = if child_pid == 0 { "(child)" } else { "(parent)" };
    println!("PID={} {} idata={} istack={}",
	     unsafe { getpid() },
	     who,
	     IDATA.load(Ordering::SeqCst),
	     istack);

    unsafe { exit(EXIT_SUCCESS) };	     
}
