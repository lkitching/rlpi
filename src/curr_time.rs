use std::ptr;

use libc::{time, localtime};

use crate::util::{fmt_strftime};

pub fn curr_time(format: &str) -> String {
    let t = unsafe { time(ptr::null_mut()) };
    let tm = unsafe { localtime(&t) };
    if tm.is_null() {
	"".to_owned()
    } else {	
	fmt_strftime(format, & unsafe { *tm }).expect("Failed to format time")
    }
}
