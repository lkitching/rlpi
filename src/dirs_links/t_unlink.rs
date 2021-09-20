//listing 18-1 (page 347)
use std::ffi::{CString};
use std::mem;
use std::os::raw::{c_char};

use libc::{exit, EXIT_SUCCESS, open, O_WRONLY, O_CREAT, O_EXCL, S_IRUSR, S_IWUSR, unlink, system, close, malloc, size_t, write, ssize_t};

use crate::error_functions::{usage_err, err_exit};

pub fn main(args: &[String]) -> ! {
    if args.len() < 2 {
        usage_err(&format!("{} temp-file [num-1kB-blocks]\n", args[0]));
    }

    let num_blocks = if args.len() > 2 {
        args[2].parse().unwrap()
    } else {
        100000
    };

    let path_s = CString::new(args[1].as_str()).expect("Failed to create CString");
    let fd = unsafe { open(path_s.as_ptr(), O_WRONLY | O_CREAT | O_EXCL, S_IRUSR | S_IWUSR) };

    if fd == -1 {
        err_exit("unlink");
    }

    //remove temp file
    if unsafe { unlink(path_s.as_ptr()) } == -1 {
        err_exit("unlink");
    }

    //write junk to file
    {
        let buf_size = 1024;
        let num_bytes = buf_size * mem::size_of::<c_char>();
        let buf = unsafe { malloc(num_bytes as size_t) };

        if buf.is_null() {
            err_exit("malloc");
        }

        for j in 0..num_blocks {
            let bytes_written = unsafe { write(fd, buf, num_bytes) };
            if bytes_written != (num_bytes as ssize_t) {
            err_exit("write");
            }
        }
    }
    

    let cmd_str = format!("df -k `dirname {}`", args[1]);
    let cmd_s = CString::new(cmd_str.as_str()).expect("Failed to create CString");
    unsafe {
        system(cmd_s.as_ptr());
    };

    if unsafe { close(fd) } == -1 {
        err_exit("close");
    }
    println!("********** Closed file descriptor");

    unsafe {
        system(cmd_s.as_ptr());
    }
		      
    unsafe { exit(EXIT_SUCCESS); }
}
