use std::os::raw::{c_char};

#[link(name = "c")]
extern {
    pub fn dirname(path: *mut c_char) -> *mut c_char;
    pub fn basename(path: *mut c_char) -> *mut c_char;
}
