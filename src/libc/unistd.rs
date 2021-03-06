use std::os::raw::{c_int, c_void, c_char};
use super::sys::types::{size_t, ssize_t, off_t};

//also defined in stdio.h
pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;
pub const SEEK_DATA: c_int = 3;
pub const SEEK_HOLE: c_int = 4;

//standard file descriptors
pub const STDIN_FILENO: c_int = 0;
pub const STDOUT_FILENO: c_int = 1;
pub const STDERR_FILENO: c_int = 2;

#[link(name = "c")]
extern {
    pub fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    pub fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    pub fn close(fd: c_int) -> c_int;
    pub fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;

    pub fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: off_t) -> c_int;
    pub fn pwrite(fd: c_int, buf: *const c_void, count: size_t, offset: off_t) -> c_int;
    
    pub fn dup(old_fd: c_int) -> c_int;
    pub fn dup2(old_fd: c_int, new_fd: c_int) -> c_int;
    pub fn dup3(old_fd: c_int, new_fd: c_int, flags: c_int) -> c_int;

    pub fn crypt(key: *const c_char, salt: *const c_char) -> *mut c_char;
}
