use crate::libc::{environ};
use std::ffi::{CStr};

//displays each environment variable
pub fn display_env() {
    unsafe {
	let mut ep = environ;
	while ! (*ep).is_null() {
	    let cs = CStr::from_ptr(*ep);
	    let env_s = cs.to_str().expect("Failed to convert into str");
	    println!("{}", env_s);
	    ep = ep.offset(1);
	}	
    }
}
