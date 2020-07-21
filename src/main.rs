use std::env;

mod libc;
mod error_functions;
mod ename;
mod fileio;

use crate::libc::{gnu_get_libc_version, read_char_ptr};
//use crate::error_functions::{terminate};
use crate::fileio::copy;

fn main() {
    unsafe {
	let c_buf = gnu_get_libc_version();
	let version_string = read_char_ptr(c_buf);
	println!("glibc version: {}", version_string);

	let args: Vec<String> = env::args().collect();
	copy::main(&args[..]);
	//terminate(true);
    }    
}
