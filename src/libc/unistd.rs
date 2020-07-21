use std::os::raw::{c_int, c_void};
use super::sys::types::{size_t, ssize_t};

#[link(name = "c")]
extern {
    pub fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    pub fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    pub fn close(fd: c_int) -> c_int;
}
