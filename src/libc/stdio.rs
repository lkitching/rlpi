use libc::{FILE};

#[link(name = "c")]
extern {
    pub static stdout: *mut FILE;
}
