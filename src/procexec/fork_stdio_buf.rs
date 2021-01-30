//listing 25-2 (page 537)

use std::ffi::{CString};
use std::os::raw::{c_void};

use libc::{write, STDOUT_FILENO, exit, EXIT_SUCCESS, fork};

use crate::error_functions::{err_exit};

pub fn main(args: &[String]) -> ! {
    println!("Hello world");
    {
	let cs = CString::new("Ciao").expect("Failed to create CString");
	unsafe { write(STDOUT_FILENO, cs.as_ptr() as *const c_void, 5); }
    }

    if unsafe { fork() } == -1 {
	err_exit("fork");
    }

    // both parent and child continue execution here
    unsafe { exit(EXIT_SUCCESS); }
}
