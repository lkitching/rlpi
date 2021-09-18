//listing 6.4 (page 131)

use std::ffi::{CString};
use libc::{clearenv, putenv, setenv, unsetenv, exit, EXIT_SUCCESS};
use crate::util::{display_env};
use crate::error_functions::{err_exit};

pub fn main(args: &[String]) -> ! {
    unsafe { clearenv(); }

    for arg in &args[1..] {
	let carg = CString::new(arg.as_str()).expect("Failed to create CString");
	let r = unsafe { putenv(carg.into_raw()) };	
	if r != 0 {
	    err_exit(&format!("putenv: {}", arg));
	}
    }

    let k = CString::new("GREET").expect("Failed to create CString");
    let v = CString::new("Hello world").expect("Failed to create CString");
    if unsafe { setenv(k.as_ptr(), v.as_ptr(), 0) } == -1 {
	err_exit("setenv");
    }

    let uk = CString::new("BYE").expect("Failed to create CString");
    unsafe { unsetenv(uk.as_ptr()); }

    display_env();	

    unsafe { exit(EXIT_SUCCESS); }
}
