//listing 26-4 (page 554)
use std::ptr;
use std::thread;
use std::time::Duration;
use std::ffi::CString;
use std::path::{Path};

use libc::{setbuf, getpid, _exit, exit, EXIT_SUCCESS, fork, kill, SIGKILL, system};

use crate::libc::stdio::{stdout};
use crate::error_functions::{err_exit, err_msg};

fn ps(prog_name: &str) {
    let cmd = format!("ps | grep {}", prog_name);
    let cmd_s = CString::new(cmd).expect("Failed to create CString");
    unsafe {
	system(cmd_s.as_ptr());
    }
}

pub fn main(args: &[String]) -> ! {
    // disable buffering of stdout
    unsafe { setbuf(stdout, ptr::null_mut()) };

    println!("Parent PID={}", unsafe { getpid() });
    let prog_name = Path::new(&args[0]).file_name().and_then(|f| f.to_str()).expect("Could not get program name");
    println!("Prog name: {}", prog_name);

    let child_pid = unsafe { fork() };
    match child_pid {
	-1 => {
	    err_exit("fork");
	},
	0 => {
	    // child process
	    // exit immediately to become a zombie
	    println!("Child (PID = {}) exiting", unsafe { getpid() });
	    unsafe { _exit(EXIT_SUCCESS) };
	},
	_ => {
	    // parent
	    // give child a chance to exit
	    thread::sleep(Duration::from_secs(3));

	    // view zombie child
	    ps(prog_name);

	    // send the 'sure kill' signal to the zombie
	    if unsafe { kill(child_pid, SIGKILL) == -1 } {
		err_msg("kill");
	    }

	    // give child a chance to react to the signal
	    thread::sleep(Duration::from_secs(3));

	    // view zombie child again
	    ps(prog_name);

	    unsafe { exit(EXIT_SUCCESS) };
	}
    }
}
