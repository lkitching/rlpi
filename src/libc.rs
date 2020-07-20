use std::ffi::CStr;
use std::os::raw::{c_int, c_char};

#[link(name = "c")]
extern {
    pub fn gnu_get_libc_version() -> *const c_char;
    pub fn strerror(errnum: c_int) -> *mut c_char;
    pub fn abort() -> !;
    pub fn exit(status: c_int) -> !;
    pub fn _exit(status: c_int) -> !;
    fn __errno_location() -> *mut c_int;
}

//TODO: move this?
pub fn read_char_ptr(chars: *const c_char) -> String {
    unsafe {
	let c_str = CStr::from_ptr(chars);
	let s = c_str.to_str().expect("Expected valid UTF-8");    
	s.to_owned()
    }
}

pub fn errno() -> c_int {
    unsafe {
	let loc = __errno_location();
	*loc
    }
}

pub fn set_errno(errno: c_int) {
    unsafe {
	let loc = __errno_location();
	*loc = errno;
    }
}



