use std::os::raw::{c_int, c_long};

#[allow(non_camel_case_types)]
pub type size_t = usize;	//from libc crate

#[allow(non_camel_case_types)]
pub type ssize_t = c_int;	//alias depends on word size? __SSIZE_T defined as __SWORD_TYPE

#[allow(non_camel_case_types)]
pub type off_t = c_long;	//__OFF_T_TYPE defined as __SYSCALL_SLONG_TYPE
