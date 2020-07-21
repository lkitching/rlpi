use std::ffi::CStr;
use std::os::raw::{c_int, c_char, c_void};

pub type mode_t = u32; //typesizes.h
pub type size_t = usize;	//from libc crate
pub type ssize_t = c_int;	//alias depends on word size? __SSIZE_T defined as __SWORD_TYPE

#[link(name = "c")]
extern {
    pub fn gnu_get_libc_version() -> *const c_char;
    pub fn strerror(errnum: c_int) -> *mut c_char;
    pub fn abort() -> !;
    pub fn exit(status: c_int) -> !;
    pub fn _exit(status: c_int) -> !;
    fn __errno_location() -> *mut c_int;

    pub fn open(pathname: *const c_char, flags: c_int, mode: mode_t) -> c_int;
    pub fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    pub fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    pub fn close(fd: c_int) -> c_int;
}

pub unsafe fn open2(pathname: *const c_char, flags: c_int) -> c_int {
    open(pathname, flags, 0)
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



