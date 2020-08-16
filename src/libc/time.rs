use std::os::raw::{c_char};
use libc::{tm, time_t};

#[link(name = "c")]
extern {
    pub fn asctime(tm: *const tm) -> *const c_char;
    pub fn ctime(timep: *const time_t) -> *const c_char;
}
