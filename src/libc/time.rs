use std::os::raw::{c_char};
use std::ffi::{CStr};
use libc::{tm, time_t, size_t, clock_t};

//NOTE: defined in bits/time.h
pub const CLOCKS_PER_SEC: clock_t = 1000000;

#[link(name = "c")]
extern {
    pub fn asctime(tm: *const tm) -> *const c_char;
    pub fn ctime(timep: *const time_t) -> *const c_char;
    pub fn strftime(s: *mut c_char, max: size_t, format: *const c_char, tm: *const tm) -> size_t;
    pub fn strptime(s: *const c_char, format: *const c_char, tm: *mut tm) -> *mut c_char;
    pub fn clock() -> clock_t;
}

pub fn ctime_string(tm: *const time_t) -> String {
    let sp = unsafe { ctime(tm) };
    let cs = unsafe { CStr::from_ptr(sp) };
    cs.to_str().expect("Invalid utf-8").to_string()
}
