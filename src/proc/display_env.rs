//based on listing 6.3 (page 127)
use std::os::raw::{c_char};
use std::ffi::{CString, CStr};
use libc::{exit, EXIT_SUCCESS};

#[link(name = "c")]
extern {
    pub static environ: *mut *mut c_char;
}

pub fn main(args: &[String]) -> ! {
    unsafe {
	let mut ep = environ;
	while ! (*ep).is_null() {
	    let cs = CStr::from_ptr(*ep);
	    let env_s = cs.to_str().expect("Failed to convert into str");
	    println!("{}", env_s);
	    ep = ep.offset(1);
	}	
	exit(EXIT_SUCCESS);
    }
}
