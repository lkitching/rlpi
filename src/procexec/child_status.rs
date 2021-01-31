//listing 26-3 (page 548)
use libc::{fork, exit, EXIT_SUCCESS, EXIT_FAILURE, waitpid, WUNTRACED, WCONTINUED, WIFSIGNALED, WIFEXITED, pause, getpid};

use crate::error_functions::{usage_err, err_exit};
use super::print_wait_status::{print_wait_status};

pub fn main(args: &[String]) {
    if args.len() > 1 && args[1] == "--help" {
	usage_err(&format!("{} [exit-status]", args[0]));
    }

    match unsafe { fork() } {
	-1 => {
	    err_exit("fork");
	},
	0 => {
	    println!("Child started with PID = {}", unsafe { getpid() });
	    
	    // child: either exits immediately with the given status
	    // or loops waiting for signals
	    if args.len() > 1 {
		let status = args[1].parse().expect("Invalid exit status");
		unsafe { exit(status); }
	    } else {
		loop {
		    unsafe { pause(); }
		}		
	    }
	    // NOTE: not reached
	    unsafe { exit(EXIT_FAILURE); }
	},
	_ => {
	    // parent - repeatedly wait on child until it exits or is terminated by a signal
	    loop {
		let mut status = -1;
		let child_pid = unsafe { waitpid(-1, &mut status, WUNTRACED | WCONTINUED) };

		if child_pid == -1 {
		    err_exit("waitpid");
		}

		// print status in hex and as separate decimal bytes
		println!("waitpid() returned: PID = {}, status = {:#x} ({}, {})",
			 child_pid,
			 status,
			 status >> 8,
			 status & 0xFF);
		print_wait_status(None, status);

		unsafe {
		    if WIFEXITED(status) || WIFSIGNALED(status) {
			exit(EXIT_SUCCESS);
		    }
		}		
	    }
	}
    }
}
