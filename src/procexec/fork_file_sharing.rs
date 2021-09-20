//listing 24-2 (page 518)
use std::ptr;
use std::ffi::{CString};
use std::os::raw::{c_int};

use libc::{exit, EXIT_SUCCESS, setbuf, lseek, SEEK_CUR, mkstemp, fcntl, F_GETFL, O_APPEND, fork,
           SEEK_SET, _exit, F_SETFL, wait};

use crate::libc::stdio::{stdout};
use crate::error_functions::{err_exit};

fn print_append_flag(fd: c_int, msg: &str) {
    let flags = unsafe { fcntl(fd, F_GETFL) };
    if flags == -1 {
	err_exit("fcntl - F_GETFL");
    }

    let has_append = O_APPEND & flags == O_APPEND;
    println!("O_APPEND flag {} is: {}", msg, if has_append { "on" } else { "off" });
}

pub fn main(args: &[String]) -> ! {
    //turn off buffering of stdout
    unsafe { setbuf(stdout, ptr::null_mut()); }

    let fd = {
		let template_s = CString::new("/tmp/testXXXXXX").expect("Invalid CString");
		let p = template_s.into_raw();
		let fd = unsafe { mkstemp(p) };
		let _ = unsafe { CString::from_raw(p) };
		fd
    };
    if fd == -1 {
		err_exit("mkstemp");
    }

    println!("File offset before fork(): {}", unsafe { lseek(fd, 0, SEEK_CUR) });

    print_append_flag(fd, "before fork()");

    let child_pid = unsafe { fork() };
    match child_pid {
	-1 => {
	    err_exit("fork");
	},
	0 => {
	    // child
	    if unsafe { lseek(fd, 1000, SEEK_SET) } == -1 {
		err_exit("lseek");
	    }

	    //fetch current flags and set O_APPEND
	    let mut flags = unsafe { fcntl(fd, F_GETFL) };
	    flags = flags | O_APPEND;

	    if unsafe { fcntl(fd, F_SETFL, flags) } == -1 {
		err_exit("fcntl - F_SETL");
	    }
	    unsafe { _exit(EXIT_SUCCESS); }
	},
	_ => {
	    // parent
	    // wait for child to exit
	    let mut child_status = 0;
	    if unsafe { wait(&mut child_status) } == -1 {
		err_exit("wait");
	    }
	    println!("Child exited with status {}", child_status);

	    println!("File offset in parent: {}", unsafe { lseek(fd, 0, SEEK_CUR) });

	    print_append_flag(fd, "in parent");
	    unsafe { exit(EXIT_SUCCESS); }
	}
    }
}
