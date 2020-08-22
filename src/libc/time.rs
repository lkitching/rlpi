use std::os::raw::{c_char};
use libc::{tm, time_t, size_t, clock_t};

#[link(name = "c")]
extern {
    pub fn asctime(tm: *const tm) -> *const c_char;
    pub fn ctime(timep: *const time_t) -> *const c_char;
    pub fn strftime(s: *mut c_char, max: size_t, format: *const c_char, tm: *const tm) -> size_t;
    pub fn strptime(s: *const c_char, format: *const c_char, tm: *mut tm) -> *mut c_char;
    pub fn clock() -> clock_t;
}
