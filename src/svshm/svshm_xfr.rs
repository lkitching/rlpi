use std::os::raw::c_int;

use libc::{S_IRUSR, S_IWUSR, S_IRGRP, S_IWGRP, key_t};

pub const SHM_KEY: key_t = 0x1234;
pub const SEM_KEY: key_t = 0x5678;

pub const OBJ_PERMS: c_int = (S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP) as c_int;

pub const WRITE_SEM: c_int = 0;
pub const READ_SEM: c_int = 1;

pub const BUF_SIZE: usize = 1024;

#[repr(C)]
pub struct ShmSeg {
    pub count: usize,
    pub buf: [u8; BUF_SIZE]
}