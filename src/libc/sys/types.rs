use std::os::raw::{c_int};

pub type size_t = usize;	//from libc crate
pub type ssize_t = c_int;	//alias depends on word size? __SSIZE_T defined as __SWORD_TYPE
