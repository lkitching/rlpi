use libc::{time, localtime};
use std::ptr;
use crate::util::{fmt_strftime};

pub fn curr_time(format: Option<&str>) -> Result<String, ()> {
    let t = unsafe { time(ptr::null_mut()) };
    let tm = unsafe { localtime(&t) };

    if tm.is_null() {
        //TODO: describe error?
        Err(())
    } else {
        let fs = format.unwrap_or("%c");
        fmt_strftime(fs, &unsafe { *tm })
    }
}
