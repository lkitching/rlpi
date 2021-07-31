use std::ffi::{CStr, CString};
use std::mem;
use std::fmt;
use std::str::{FromStr};
use libc::{tm, fork};
use std::os::raw::{c_char, c_int};

use crate::libc::{environ};
use crate::libc::time::{strftime};
use crate::error_functions::{err_exit};

//displays each environment variable
pub fn display_env() {
    unsafe {
	let mut ep = environ;
	while ! (*ep).is_null() {
	    let cs = CStr::from_ptr(*ep);
	    let env_s = cs.to_str().expect("Failed to convert into str");
	    println!("{}", env_s);
	    ep = ep.offset(1);
	}	
    }
}

fn vec_i8_into_u8(v: Vec<i8>) -> Vec<u8> {
    // prevent destructor for v from running since the ownership of the
    // internal buffer is about to be moved
    let mut v = mem::ManuallyDrop::new(v);

    // get pointer to internal buffer - this pointer will be cast from
    // *i8 to *u8 when constructing the new buffer
    let p = v.as_mut_ptr();
    unsafe { Vec::from_raw_parts(p as *mut u8, v.len(), v.capacity()) }    
}

pub fn vec_u8_into_i8(v: Vec<u8>) -> Vec<i8> {
    let mut v = mem::ManuallyDrop::new(v);
    let p = v.as_mut_ptr();
    unsafe { Vec::from_raw_parts(p as *mut i8, v.len(), v.capacity()) }
}

pub fn fmt_strftime(format: &str, tm: &tm) -> Result<String, ()> {
    //TODO: propagate error?
    let fs_cstr = CString::new(format).expect("Failed to create CString");

    let buf_size = 1000;
    let mut buf = Vec::with_capacity(buf_size);
    let s = unsafe { strftime(buf.as_mut_ptr(), buf_size, fs_cstr.as_ptr(), tm) };

    if s == 0 {
	Err(())
    } else {
	unsafe { buf.set_len(s); }
	buf.shrink_to_fit();
	let buf = vec_i8_into_u8(buf);
	let cs = unsafe { CString::from_vec_unchecked(buf) };
	let s = cs.to_str().expect("Failed to create str").to_owned();
	Ok(s)
    }
}

//read a str reference from a buffer of C chars
//WARNING: declared lifetime of returned slice is arbitrary so buffer
//must outlive it!
pub fn read_str<'a>(chars: *const c_char) -> &'a str {
    unsafe {
	CStr::from_ptr(chars).to_str().expect("Invalid CStr")
    }
}

pub fn numeric_arg_or<T>(args: &[String], index: usize, default: T) -> T where
  T : FromStr,
  T::Err : fmt::Debug {
    if index < args.len() {
	T::from_str(args[index].as_str()).expect("Invalid number")
    } else {
	default
    }
}

pub enum ForkResult {
    Parent(c_int),
    Child
}

pub fn fork_or_die() -> ForkResult {
    match try_fork() {
	Ok(r) => { r },
	Err(msg) => {
	    err_exit(&msg);
	}
    }
}

pub fn try_fork() -> Result<ForkResult, String> {
    let child_pid = unsafe { fork() };
    match child_pid {
	-1 => { Err(String::from("fork failed")) },
	0 => { Ok(ForkResult::Child) },
	_ => { Ok(ForkResult::Parent(child_pid)) }
    }
}
