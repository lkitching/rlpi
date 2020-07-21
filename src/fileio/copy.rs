use std::ffi::{CString};
use std::os::raw::{c_int, c_void};

use crate::libc::{exit, ExitStatus};
use crate::libc::unistd::{read, write, close};
use crate::libc::sys::types::{size_t};
use crate::libc::sys::stat::{S_IRUSR, S_IWUSR, S_IRGRP, S_IWGRP, S_IROTH, S_IWOTH};
use crate::libc::fcntl::{open, open2, O_CREAT, O_WRONLY, O_TRUNC, O_RDONLY};
use crate::error_functions::{usage_err, err_exit, fatal};


pub fn main(args: &[String]) -> ! {
    if args.len() != 3 {
	//NOTE: args should always be non-empty
	usage_err(&format!("{} old-file new-file\n", args[0]));
    }

    let src_path = args[1].as_str();
    let csrc_path = CString::new(src_path).expect("Failed to create CString");
    
    let input_fd = unsafe { open2(csrc_path.as_ptr(), O_RDONLY) };
    if input_fd == -1 {
	err_exit(&format!("opening file {}", src_path));
    }

    let open_flags = O_CREAT | O_WRONLY | O_TRUNC;
    let file_perms = S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH; //rw-rw-rw-

    let dest_path = args[2].as_str();
    let cdest_path = CString::new(dest_path).expect("Failed to create CString");
    let output_fd = unsafe { open(cdest_path.as_ptr(), open_flags, file_perms) };

    if output_fd == -1 {
	err_exit(&format!("opening file {}", dest_path));
    }

    //transfer data until end of input (or error) is reached
    let mut buf: [u8; 1024] = [0; 1024];
    let mut num_read = unsafe { read(input_fd, buf.as_mut_ptr() as *mut c_void, buf.len()) };
    while num_read > 0 {
	let written = unsafe { write(output_fd, buf.as_mut_ptr() as *mut c_void, num_read as size_t) };
	if written != num_read {
	    fatal("couldn't write whole buffer");
	}
	num_read = unsafe { read(input_fd, buf.as_mut_ptr() as *mut c_void, buf.len()) };
    }

    if num_read == -1 {
	err_exit("read");
    }

    if unsafe { close(input_fd) } == -1 {
	err_exit("close input");
    }

    if unsafe { close(output_fd) } == -1 {
	err_exit("close output");
    }

    unsafe { exit(ExitStatus::Success as c_int); }
}
