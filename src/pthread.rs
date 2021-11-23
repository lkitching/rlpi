use std::os::raw::{c_void};

pub const PTHREAD_CANCELED: *mut c_void = (-1 as isize) as *mut c_void;
