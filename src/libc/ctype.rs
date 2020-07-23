use std::os::raw::{c_int};

#[link(name = "c")]
extern {
    pub fn isprint(c: c_int) -> c_int;
}
