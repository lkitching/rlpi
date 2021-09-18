//listing 18-4 (page 369)
use std::mem::{MaybeUninit};
use std::ffi::{CString, CStr};
use std::os::raw::{c_char};

use libc::{exit, EXIT_SUCCESS, lstat, S_IFMT, S_IFLNK, PATH_MAX, readlink, realpath};

use crate::error_functions::{usage_err, err_exit, fatal};

pub fn main(args: &[String]) -> ! {
    if args.len() != 2 {
	usage_err(&format!("{} pathname\n", args[0]));
    }

    let mut statbuf = MaybeUninit::uninit();
    let path = args[1].as_str();
    let path_s = unsafe { CString::new(path).expect("Failed to create CString") };

    if unsafe { lstat(path_s.as_ptr(), statbuf.as_mut_ptr()) } == -1 {
	err_exit("lstat");
    }

    let statbuf = unsafe { statbuf.assume_init() };
    let is_link = (statbuf.st_mode & S_IFMT) == S_IFLNK;

    if ! is_link {
	fatal(&format!("{} is not a symbolic link", path));
    }

    let mut buf: [c_char; PATH_MAX as usize] = [0; PATH_MAX as usize];
    let num_bytes = unsafe { readlink(path_s.as_ptr(), buf.as_mut_ptr(), buf.len() - 1) };
    if num_bytes == -1 {
	err_exit("readlink");
    }
    buf[num_bytes as usize] = '\0' as c_char;

    {
	let link_s = unsafe { CStr::from_ptr(buf.as_ptr()).to_str().expect("Failed to read CStr") };
	println!("readlink: {} --> {}", path, link_s);
    }

    if unsafe { realpath(path_s.as_ptr(), buf.as_mut_ptr()) }.is_null() {
	err_exit("realpath");
    }

    {
	let real_s = unsafe { CStr::from_ptr(buf.as_ptr()).to_str().expect("Failed to read CStr") };
	println!("realpath: {} --> {}", path, real_s);
    }
    
    unsafe { exit(EXIT_SUCCESS); }
}
