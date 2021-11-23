use std::os::raw::{c_int, c_char};
use libc::{FILE};

#[link(name = "c")]
extern {
    pub static stdout: *mut FILE;
    pub static _sys_nerr: c_int;

    // warning: length is a hack!
    pub static mut _sys_errlist: [*const c_char; 100];
}
