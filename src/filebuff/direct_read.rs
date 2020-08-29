//listing 13-1 (page 247)
use std::ffi::{CString};
use libc::{exit, EXIT_SUCCESS, open, O_RDONLY, O_DIRECT, lseek, SEEK_SET, memalign, size_t, read};

use crate::error_functions::{usage_err, err_exit};

pub fn main(args: &[String]) -> ! {
    if args.len() < 3 {
	usage_err(&format!("{} file length [offset [alignment]]", args[0]));
    }

    let length: size_t = args[2].parse().expect("Invalid length - expected number");
    let offset = if args.len() > 3 {
	args[3].parse().expect("Invalid offset - expected number")
    } else {
	0
    };
    
    let alignment = if args.len() > 4 {
	args[4].parse().expect("Invalid alignment - expected number")
    } else {
	4096
    };

    let path_str = CString::new(args[1].clone()).expect("Failed to create CString");
    let fd = unsafe { open(path_str.as_ptr(), O_RDONLY | O_DIRECT) };
    if fd == -1 {
	err_exit("open");
    }

    // memalign() allocates a block of memory aligned on an address
    // that is a multiple of its first argument. By specifying this
    // argument as 2 * alignment and then adding alignment to the
    // returned pointer, we ensure that buf is aligned on a
    // non-power-of-two mutliple of alignment. We do this to ensure
    // that if, for example, we ask for a 256 byte-aligned buffer, we
    // don't accidentally get a buffer that is also aligned on a
    // 512-byte boundary.

    let mut buf = unsafe { memalign(alignment * 2, length + alignment) };
    if buf.is_null() {
	err_exit("memalign");
    }

    buf = unsafe { buf.add(alignment) };

    if unsafe { lseek(fd, offset, SEEK_SET) } == -1 {
	err_exit("lseek");
    }

    let num_read = unsafe { read(fd, buf, length) };
    if num_read == -1 {
	err_exit("read");
    }

    println!("Read {} bytes", num_read);

    unsafe { exit(EXIT_SUCCESS); }
}
