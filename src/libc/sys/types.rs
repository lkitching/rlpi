use std::os::raw::{c_int, c_long};

pub type size_t = usize;	//from libc crate
pub type ssize_t = c_int;	//alias depends on word size? __SSIZE_T defined as __SWORD_TYPE
pub type off_t = c_long;	//__OFF_T_TYPE defined as __SYSCALL_SLONG_TYPE
