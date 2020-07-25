//exercise 5.2

use std::ffi::{CString};
use std::os::raw::{c_void};
use libc::{open, O_WRONLY, O_APPEND, exit, EXIT_SUCCESS, write, size_t, lseek, SEEK_SET};
use crate::error_functions::{usage_err, err_exit};

pub fn main(args: &[String]) -> ! {
    if args.len() != 2 {
	usage_err(&format!("{} file", args[0]));
    }

    let dest_path = args[1].as_str();
    let cdest_path = CString::new(dest_path).expect("Failed to create CString");
    let fd = unsafe { open(cdest_path.as_ptr(), O_WRONLY | O_APPEND) };

    if fd == -1 {
	err_exit(&format!("Failed to open {} for writing", dest_path));
    }

    let to_write = CString::new("the quick brown fox jumped over the lazy dog").expect("Failed to create CString");
    let bytes = to_write.into_bytes();

    unsafe { lseek(fd, 0, SEEK_SET); }
    let written = unsafe { write(fd, bytes.as_ptr() as *const c_void, bytes.len()) };

    if written == -1 {
	err_exit("write failed");
    }

    if (written as size_t) != bytes.len() {
	err_exit("failed to write all bytes");
    }

    unsafe { exit(EXIT_SUCCESS); }
}
