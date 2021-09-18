//listing 44-4 (page 900)

use std::{ptr};
use std::ffi::{CString};
use std::os::raw::{c_char};

use libc::{close, dup2, STDOUT_FILENO, STDIN_FILENO, execlp, wait, exit, EXIT_SUCCESS};

extern crate rlpi;
use rlpi::util::{create_pipe, ForkResult, fork_or_die};
use rlpi::error_functions::{err_exit};

pub fn main() {
	let pipe = create_pipe().expect("Failed to create pipe");

    if let ForkResult::Child = fork_or_die() {
	// first child
	// exec 'ls' to write to pipe
	
	// read side of pipe is unused
	if unsafe { close(pipe.read_fd) } == -1 {
	    err_exit("close 1");
	}

	// duplicate stdout on the write end of the pipe and close the
	// duplicated descriptor
	if pipe.write_fd != STDOUT_FILENO {
	    if unsafe { dup2(pipe.write_fd, STDOUT_FILENO) } == -1 {
		err_exit("dup2 1");
	    }

	    if unsafe { close(pipe.write_fd) } == -1 {
		err_exit("close 2");
	    }
	}

	// exec 'ls' which writes its output to the pipe
	let cmd_s = CString::new("ls").expect("Failed to create CString");
	unsafe {
	    execlp(cmd_s.as_ptr(), cmd_s.as_ptr(), ptr::null() as *const c_char);
	}

	// NOTE: only gets here if execlp fails
	err_exit("execlp ls");
    }

    // parent falls through to create next child
    if let ForkResult::Child = fork_or_die() {
	// second child
	// exec 'wc' to read from the pipe
	// write end of pipe is unused
	if unsafe { close(pipe.write_fd) } == -1 {
	    err_exit("close 3");
	}

	// duplicate stdin on the read end of the pipe
	if pipe.read_fd != STDIN_FILENO {
	    if unsafe { dup2(pipe.read_fd, STDIN_FILENO) } == -1 {
		err_exit("dup2 2");
	    }
	    
	    if unsafe { close(pipe.read_fd) } == -1 {
		err_exit("close 4");
	    }
	}

	let cmd_s = CString::new("wc").expect("Failed to create CString");
	let arg_s = CString::new("-l").expect("Failed to create CString");
	unsafe { execlp(cmd_s.as_ptr(), cmd_s.as_ptr(), arg_s.as_ptr(), ptr::null() as *const c_char); }

	// NOTE: only gets here if execlp fails
	err_exit("execlp wc");
    }

    // parent continues here
    // close unused file descriptors for pipe
    if unsafe { close(pipe.read_fd) } == -1 {
	err_exit("close 5");
    }

    if unsafe { close(pipe.write_fd) } == -1 {
	err_exit("close 6");
    }

    // wait for children
    if unsafe { wait(ptr::null_mut()) } == -1 {
	err_exit("wait 1");
    }

    if unsafe { wait(ptr::null_mut()) } == -1 {
	err_exit("wait 2");
    }

    unsafe { exit(EXIT_SUCCESS); }
}
