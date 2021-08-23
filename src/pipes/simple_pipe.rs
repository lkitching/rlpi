//listing 44-2 (page 896)
use std::env;
use std::os::raw::{c_int, c_void};
use std::ptr;

use libc::{pipe, exit, _exit, EXIT_SUCCESS, close, read, write, STDOUT_FILENO, wait};

extern crate rlpi;

use rlpi::error_functions::{usage_err, err_exit, fatal};
use rlpi::util::{ForkResult, fork_or_die};

pub fn main() {
    let args: Vec<String> = env::args().collect();
    const BUF_SIZE: usize = 10;

    if args.len() != 2 || args[1] == "--help" {
	usage_err(&format!("{} message", args[0]));
    }

    let mut pipe_fds: [c_int; 2] = [0, 0];
    if unsafe { pipe(pipe_fds.as_mut_ptr()) } == -1 {
	err_exit("pipe");
    }

    match fork_or_die() {
	ForkResult::Child => {
	    // write end of pipe is unused in the child
	    if unsafe { close(pipe_fds[1]) } == -1 {
		err_exit("close - child");
	    }

	    // read data from the pipe, echo on stdout
	    let mut buf: [u8; BUF_SIZE] = [0; BUF_SIZE];
	    loop {
		let bytes_read = unsafe { read(pipe_fds[0], buf.as_mut_ptr() as *mut c_void, BUF_SIZE) };
		match bytes_read {
		    -1 => { err_exit("read"); },
		    0 => {
			// end of file
			break;
		    },
		    _ => {
			if unsafe { write(STDOUT_FILENO, buf.as_ptr() as *const c_void, bytes_read as usize) } != bytes_read {
			    fatal("child - partial/failed write");
			}
		    }
		}
	    }

	    unsafe { write(STDOUT_FILENO, "\n".as_ptr() as *const c_void, 1); }
	    if unsafe { close(pipe_fds[0]) } == -1 {
		err_exit("close");
	    }
	    unsafe { _exit(EXIT_SUCCESS); }
	},
	ForkResult::Parent(child_pid) => {
	    // parent writes CLI argument to pipe
	    // read end is unused
	    if unsafe { close(pipe_fds[0]) } == -1 {
		err_exit("close - parent");
	    }

	    let to_write = args[1].as_bytes().len();
	    if unsafe { write(pipe_fds[1], args[1].as_ptr() as *const c_void, to_write) } != (to_write as isize) {
		fatal("parent - partial/failed write");
	    }

	    // required so child process sees EOF
	    if unsafe { close(pipe_fds[1]) } == -1 {
		err_exit("close");
	    }

	    // wait for child to finish
	    unsafe {
		wait(ptr::null_mut());
		exit(EXIT_SUCCESS);
	    }
	}
    }
}
