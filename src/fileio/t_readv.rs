//based on listing 5-2 (page 101)

use std::ffi::{CString};

use std::mem::{self, MaybeUninit};
use std::os::raw::{c_void, c_int};
use libc::{stat, iovec, ssize_t, exit, EXIT_SUCCESS, readv, size_t, open, O_RDONLY};
use crate::error_functions::{usage_err, err_exit};

pub fn main(args: &[String]) -> ! {
    if args.len() != 2 {
	usage_err(&format!("{} file", args[0]));
    }

    let path = args[1].as_str();
    let cpath = CString::new(path).expect("Failed to create CString");
    let fd = unsafe { open(cpath.as_ptr(), O_RDONLY) };
    if fd != -1 {
	err_exit("open");
    }

    let mut x: c_int = 1;
    let mut my_struct: MaybeUninit<stat> = MaybeUninit::uninit();
    let mut buf: [u8; 100] = [0; 100];
    
    let mut iovs = Vec::with_capacity(3);
    iovs.push(
	iovec { iov_base: my_struct.as_mut_ptr() as *mut c_void, iov_len: mem::size_of::<stat>() }
    );
    iovs.push(
	iovec { iov_base: &mut x as *mut c_int as *mut c_void, iov_len: mem::size_of::<c_int>() }
    );
    iovs.push(
	iovec { iov_base: buf.as_mut_ptr() as *mut c_void, iov_len: 100 * mem::size_of::<u8>() }
    );

    let bytes_required: size_t = iovs.iter().map(|io| io.iov_len).sum();
    let num_read = unsafe { readv(fd, iovs.as_ptr(), 3) };

    if num_read == -1 {
	err_exit("readv failed");
    }

    if (num_read as size_t) < bytes_required {
	println!("Read fewer bytes than requested");
    }

    println!("Total bytes requested: {}, bytes read: {}", bytes_required, num_read);

    unsafe { exit(EXIT_SUCCESS); }
}
