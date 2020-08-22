//listing 11-2 (page 218)
use std::os::raw::{c_int};
use libc::{fpathconf, exit, EXIT_SUCCESS, _PC_NAME_MAX, _PC_PATH_MAX, _PC_PIPE_BUF, STDIN_FILENO};

use crate::libc::{errno, set_errno};
use crate::error_functions::{err_exit};

fn fpathconf_print(msg: &str, fd: c_int, name: c_int) {
    // set errno to 0 before calling fpathconf so failure can be
    // distinguished from an indeterminate value
    set_errno(0);
    let lim = unsafe { fpathconf(fd, name) };
    if lim != -1 {
	println!("{} {}", msg, lim);
    } else {
	let en = unsafe { errno() };
	if en == 0 {
	    println!("{} (indeterminate)", msg);
	} else {
	    err_exit(&format!("fpathconf {}", msg));
	}
    }
}

pub fn main(args: &[String]) -> ! {    
    fpathconf_print("_PC_NAME_MAX: ", STDIN_FILENO, _PC_NAME_MAX);
    fpathconf_print("_PC_PATH_MAX: ", STDIN_FILENO, _PC_PATH_MAX);
    fpathconf_print("_PC_PIPE_BUF: ", STDIN_FILENO, _PC_PIPE_BUF);
    unsafe { exit(EXIT_SUCCESS); }
}
