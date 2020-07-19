use std::os::raw::{c_char};
use std::ffi::CStr;

#[link(name = "c")]
extern {
    fn gnu_get_libc_version() -> *const c_char;
}

fn main() {
    unsafe {
	let c_buf = gnu_get_libc_version();
	let c_str = CStr::from_ptr(c_buf);
	let str_slice = c_str.to_str().expect("Expected valid UTF-8");
	println!("glibc version: {}", str_slice);
    }    
}
