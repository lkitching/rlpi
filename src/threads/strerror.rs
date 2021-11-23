// listing 31-1 (page 664)
use std::os::raw::{c_int};

use crate::libc::stdio::{_sys_nerr, _sys_errlist};
use std::ffi::CStr;

static mut MSG_BUF: String = String::new();

pub fn strerror(err: c_int) -> &'static str {
    let p = unsafe { _sys_errlist[err as usize] };
    if err < 0 || err >= unsafe { _sys_nerr } || p.is_null() {
        unsafe { MSG_BUF = format!("Unknown error {}", err) }
    } else {
        let err_s = unsafe { CStr::from_ptr(p) };
        let err_msg = err_s.to_str().expect("Invalid utf8");
        unsafe {
            MSG_BUF.clear();
            MSG_BUF.insert_str(0, err_msg);
        }
    }
    unsafe { &MSG_BUF }
}