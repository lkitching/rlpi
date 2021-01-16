//listing 24-5 (page 526)
use std::ptr;

use libc::{exit, _exit, EXIT_SUCCESS, setbuf, wait, fork};

use crate::error_functions::{usage_err, err_exit};
use crate::libc::stdio::{stdout};

pub fn main(args: &[String]) -> ! {
    if args.len() > 1 && args[1] == "--help" {
	usage_err(&format!("{} [num-children]", args[0]));
    }

    let num_children: usize = if args.len() > 1 {
	args[1].parse().expect("Invalid number of children")
    } else {
	1
    };

    // turn of buffering of stdout
    unsafe { setbuf(stdout, ptr::null_mut()) };

    for j in (0..num_children) {
	let child_pid = unsafe { fork() };
	match child_pid {
	    -1 => {
		err_exit("fork");
	    },
	    0 => {
		println!("{} child", j);
		unsafe { _exit(EXIT_SUCCESS) };
	    },
	    _ => {
		println!("{} parent", j);
		unsafe { wait(ptr::null_mut()) };
	    }
	}
    }

    unsafe { exit(EXIT_SUCCESS); }
}
