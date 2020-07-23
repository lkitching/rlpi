use std::os::raw::{c_int, c_char};
use super::sys::stat::{mode_t};

//mask for file access modes
pub const O_ACCMODE: c_int = 0o003;

//file access modes
pub const O_RDONLY: c_int = 0o00;
pub const O_WRONLY: c_int = 0o01;
pub const O_RDWR: c_int = 0o02;

//file creation flags
pub const O_CREAT: c_int = 0o100;
pub const O_EXCL: c_int = 0o200;
pub const O_NOCTTY: c_int = 0o400;
pub const O_TRUNC: c_int = 0o1000;

//file status flags for open/fcntl
pub const O_APPEND: c_int = 0o2000;
pub const O_DSYNC: c_int = 0o10000;
pub const O_NONBLOCK: c_int = 0o4000;
pub const O_SYNC: c_int = 0o4010000;

//see fcntl-linux.h for why these are defined to be the same
pub const O_RSYNC: c_int = O_SYNC;

#[link(name = "c")]
extern {
    pub fn open(pathname: *const c_char, flags: c_int, mode: mode_t) -> c_int;
    pub fn creat(pathname: *const c_char, mode: mode_t) -> c_int;
}

pub unsafe fn open2(pathname: *const c_char, flags: c_int) -> c_int {
    open(pathname, flags, 0)
}
