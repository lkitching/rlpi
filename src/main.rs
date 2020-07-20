use std::os::raw::{c_char};
use std::ffi::CStr;

mod libc;
mod error_functions;
mod ename;

use crate::libc::{gnu_get_libc_version, read_char_ptr};
use crate::error_functions::{terminate, output_error};

fn main() {
    unsafe {
	let c_buf = gnu_get_libc_version();
	let version_string = read_char_ptr(c_buf);
	println!("glibc version: {}", version_string);

	output_error(true, 5, true, "Error occured");
	terminate(true);
    }    
}
