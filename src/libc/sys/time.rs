use std::os::raw::{c_int};

use libc::{itimerval};

#[link(name = "c")]
extern {
    pub fn getitimer(which: c_int, curr_value: *mut itimerval) -> c_int;
    pub fn setitimer(which: c_int, new_value: *const itimerval, old_value: *mut itimerval) -> c_int;
}
