use std::os::raw::{c_int, c_void};
use std::ffi::{CString};

use crate::libc::{exit, ExitStatus};
use crate::libc::sys::types::{size_t};
use crate::libc::unistd::{read, write, close, STDIN_FILENO, STDOUT_FILENO};
use crate::libc::fcntl::{open, O_CREAT, O_WRONLY, O_TRUNC};
use crate::libc::sys::stat::{S_IRUSR, S_IWUSR, S_IRGRP, S_IWGRP, S_IROTH, S_IWOTH};
use crate::error_functions::{usage_err, fatal, err_exit};

fn write_to(fd: c_int, buf: &[u8], num_bytes: size_t) {
    let bytes_written = unsafe { write(fd, buf.as_ptr() as *const c_void, num_bytes) };

    if bytes_written == -1 {
	fatal("call to write() failed");
    }
    
    if (bytes_written as size_t) != num_bytes {
	fatal("couldn't write whole buffer");
    }
}

pub fn main(args: &[String]) -> ! {
    if args.len() != 2 {
	usage_err(&format!("{} dest-file", args[0]));
    }

    let dest_path = args[1].as_str();
    let cdest_path = CString::new(dest_path).expect("Failed to create CString");

    let open_flags = O_WRONLY | O_CREAT | O_TRUNC;
    let file_perms = S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH; //rw-rw-rw-
    let dest_fd = unsafe { open(cdest_path.as_ptr(), open_flags, file_perms) };

    if dest_fd == -1 {
	err_exit(&format!("Failed to open {}", dest_path));
    }

    let mut buf: [u8; 1024] = [0; 1024];
    let mut num_read = unsafe { read(STDIN_FILENO, buf.as_mut_ptr() as *mut c_void, buf.len()) };
    while num_read > 0 {
	write_to(STDOUT_FILENO, &buf, num_read as size_t);
	write_to(dest_fd, &buf, num_read as size_t);
	num_read = unsafe { read(STDIN_FILENO, buf.as_mut_ptr() as *mut c_void, buf.len()) };
    }

    if num_read == -1 {
	err_exit("read failed");
    }

    if unsafe { close(dest_fd) } == -1 {
	err_exit("close dest");
    }

    unsafe { exit(ExitStatus::Success as c_int); }
}
