use std::ffi::CStr;
use std::os::raw::{c_int, c_char};

pub mod ctype;
pub mod fcntl;
pub mod sys;
pub mod unistd;
pub mod time;
pub mod libgen;

#[link(name = "c")]
extern {
    pub fn gnu_get_libc_version() -> *const c_char;
    pub fn strerror(errnum: c_int) -> *mut c_char;
    pub fn abort() -> !;
    pub fn exit(status: c_int) -> !;
    pub fn _exit(status: c_int) -> !;
    fn __errno_location() -> *mut c_int;
    pub static environ: *mut *mut c_char;
}


//see https://www.gnu.org/software/libc/manual/html_node/Exit-Status.html
pub enum ExitStatus {
    Success = 0,
    Failure = 1
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
