//listing 10-4 (page 199)

use std::ptr;
use libc::{setlocale, time, exit, EXIT_SUCCESS, LC_ALL, localtime};
use std::ffi::{CString, CStr};
use rlpi::util::{fmt_strftime};
use rlpi::error_functions::{err_exit, fatal};
use rlpi::libc::time::{asctime, ctime};

pub fn main() {
    let es = CString::new("").expect("Failed to create CString");
    if unsafe { setlocale(LC_ALL, es.as_ptr()) }.is_null() {
        err_exit("setlocale");
    }

    let t = unsafe { time(ptr::null_mut()) };
    let ctime_p = unsafe { ctime(&t) };
    let ctime_str = unsafe { CStr::from_ptr(ctime_p) };

    println!("ctime() of time() value is: {}", ctime_str.to_str().expect("Failed to convert CStr"));

    let loc = unsafe { localtime(&t) };
    if loc.is_null() {
        err_exit("localtime");
    }

    let asctime_p = unsafe { asctime(loc) };
    let asctime_str = unsafe { CStr::from_ptr(asctime_p) };
    println!("asctime() of local time is: {}", asctime_str.to_str().expect("Failed to convert CStr"));

    let fr = fmt_strftime("%A, %d %B %Y, %H:%M:%S %Z", &unsafe { *loc });
    match fr {
        Ok(s) => {
            println!("strftime() of local time is: {}", s);
        }
        Err(_) => {
            fatal("error calling strftime");
        }
    }

    unsafe { exit(EXIT_SUCCESS); }
}
