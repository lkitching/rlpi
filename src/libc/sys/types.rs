use std::os::raw::{c_int, c_long};

#[warn(non_camel_case_types)]
pub type size_t = usize;	//from libc crate

#[warn(non_camel_case_types)]
pub type ssize_t = c_int;	//alias depends on word size? __SSIZE_T defined as __SWORD_TYPE

#[warn(non_camel_case_types)]
pub type off_t = c_long;	//__OFF_T_TYPE defined as __SYSCALL_SLONG_TYPE
