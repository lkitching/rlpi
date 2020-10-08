//listing 18-2 (page 356)
use std::ffi::{CString, CStr};
use libc::{exit, EXIT_SUCCESS, opendir, closedir, readdir, DIR};

use crate::error_functions::{err_msg, err_exit};
use crate::libc::{errno, set_errno};

struct DirStream {
    dir: *mut DIR
}

impl Iterator for DirStream {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
	let dp = unsafe { readdir(self.dir) };
	if dp.is_null() {
	    None
	} else {
	    let ent_name = unsafe { CStr::from_ptr((*dp).d_name.as_ptr()).to_str().expect("Failed to read CStr") };
	    Some(ent_name.to_owned())
	}	
    }
}

impl Drop for DirStream {
    fn drop(&mut self) {
	if unsafe { closedir(self.dir) } == -1 {
	    err_msg("closedir");
	}
    }
}

fn open_dir(path: &str) -> DirStream {
    let dir_s = unsafe { CString::new(path).expect("Failed to create CString") };
    let dir_p = unsafe { opendir(dir_s.as_ptr()) };

    if dir_p.is_null() {
	err_msg(&format!("opendir failed on '{}'", path));
    }
    DirStream { dir: dir_p }
}

fn list_files(path: &str) {
    let is_current = path == ".";

    let stream = open_dir(path);

    for ent_name in stream {
	set_errno(0);	//to distinguish error for end-of-directory
		
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
