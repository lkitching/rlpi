use std::ffi::{CString};
use std::os::raw::{c_int, c_void};

use crate::libc::{exit, ExitStatus, open, open2, mode_t, read, write, close, size_t};
use crate::error_functions::{usage_err, err_exit, fatal};

const O_ACCMODE: c_int = 0o003;
const O_RDONLY: c_int = 0o00;
const O_WRONLY: c_int = 0o01;
const O_RDWR: c_int = 0o02;

const O_CREAT: c_int = 0o100;
const O_EXCL: c_int = 0o200;
const O_NOCTTY: c_int = 0o400;
const O_TRUNC: c_int = 0o1000;
const O_APPEND: c_int = 0o2000;

//defined in fcntl.h as aliases for types in bits/stat.h
const S_IRUSR: mode_t = 0o400;
const S_IWUSR: mode_t = 0o200;
const S_IXUSR: mode_t = 0o100;

const S_IRGRP: mode_t = S_IRUSR >> 3;
const S_IWGRP: mode_t = S_IWUSR >> 3;
const S_IXGRP: mode_t = S_IXUSR >> 3;

const S_IROTH: mode_t = S_IRGRP >> 3;
const S_IWOTH: mode_t = S_IWGRP >> 3;
const S_IXOTH: mode_t = S_IXGRP >> 3;

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

    if (output_fd == -1) {
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
