use std::os::raw::{c_int, c_void};

#[link(name = "c")]
extern {
    pub fn on_exit(cb: extern "C" fn(c_int, *const c_void), data: *const c_void) -> c_int;
}
