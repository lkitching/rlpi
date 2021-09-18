//listing 26-1 (page 543)
use std::{ptr, thread};
use std::time::{Duration};

use libc::{setbuf, fork, getpid, _exit, exit, EXIT_SUCCESS, wait, ECHILD};

use crate::error_functions::{usage_err, err_exit};
use crate::libc::{errno};
use crate::libc::stdio::{stdout};
use crate::time::curr_time::{curr_time};

pub fn main(args: &[String]) {
    if args.len() < 2 || args[1] == "--help" {
	usage_err(&format!("{} sleep-time...", args[0]));
    }

    // disable buffering of stdout
    unsafe { setbuf(stdout, ptr::null_mut()); }

    // create child process for each sleep period
    for j in 1..args.len() {
		match unsafe { fork() } {
			-1 => {
				err_exit("fork");
			},
			0 => {
				let period_seconds: u8 = args[j].parse().expect("Invalid sleep period");
				println!("[{}] child {} started with PID {}, sleeping {} seconds",
					 curr_time(Some("%T")).expect("Failed to get current time"),
					 j,
					 unsafe { getpid() },
					 period_seconds);
				thread::sleep(Duration::from_secs(period_seconds as u64));
				unsafe { _exit(EXIT_SUCCESS); }
			},
			_ => {
			// parent
			// continue spawning child processes
			}
		}
    }

    // wait for all child processes to exit
    let mut num_dead = 0;
    loop {
		let child_pid = unsafe { wait(ptr::null_mut()) };
		if child_pid == -1 {
			if errno() == ECHILD {
				println!("No more children - bye!");
				unsafe { exit(EXIT_SUCCESS); }
			} else {
				err_exit("wait");
			}
		} else {
			num_dead = num_dead + 1;
			println!("[{}] wait() returned child PID {} (num_dead = {})",
				 curr_time(Some("%T")).expect("Failed to get current time"),
				 child_pid,
				 num_dead);
		}
    }
}
