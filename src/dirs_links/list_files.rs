//listing 18-2 (page 356)
use std::ffi::{CString, CStr};
use libc::{exit, EXIT_SUCCESS, opendir, closedir, readdir};

use crate::error_functions::{err_msg, err_exit};
use crate::libc::{errno, set_errno};

fn list_files(path: &str) {
    let is_current = path == ".";

    let dir_s = unsafe { CString::new(path).expect("Failed to create CString") };
    let dir_p = unsafe { opendir(dir_s.as_ptr()) };
    if dir_p.is_null() {
	err_msg(&format!("opendir failed on '{}'", path));
    }

    loop {
	set_errno(0);	//to distinguish error for end-of-directory
	let dp = unsafe { readdir(dir_p) };
	if dp.is_null() {
	    break;
	}

	let ent_name = unsafe { CStr::from_ptr((*dp).d_name.as_ptr()).to_str().expect("Failed to read CStr") };
	if ent_name == "." || ent_name == ".." {
	    //skip . and ..
	    continue;
	}

	if !is_current {
	    print!("{}/", path);
	}
	println!("{}", ent_name);
    }

    if errno() != 0 {
	err_exit("readdir");
    }

    if unsafe { closedir(dir_p) } == -1 {
	err_msg("closedir");
    }
}

pub fn main(args: &[String]) -> ! {
    match args.len() {
	0 | 1 => {
	    list_files(".");
	},
	_ => {
	    for s in args.iter().skip(1) {
		list_files(s);
	    }
	}
    }

    unsafe { exit(EXIT_SUCCESS); }
}
