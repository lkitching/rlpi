use std::os::raw::{c_int, c_ushort, c_short};

use libc::{seminfo, semid_ds};

//defined in <sys/bits.h>
pub const GETPID: c_int = 11;
pub const GETVAL: c_int = 12;
pub const GETALL: c_int = 13;
pub const GETNCNT: c_int = 14;
pub const GETZCNT: c_int = 15;
pub const SETVAL: c_int = 16;
pub const SETALL: c_int = 17;

pub const SEM_UNDO: c_short = 0x1000;

#[repr(C)]
pub union semun {
    pub val: c_int,
    pub buf: *mut semid_ds,
    pub array: *mut c_ushort,
    pub __buf: *const seminfo
}
