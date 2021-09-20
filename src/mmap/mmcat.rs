// listing 49-1 (page 1022)
use std::{env, ptr};
use std::ffi::CString;
use std::mem::MaybeUninit;

use libc::{open, O_RDONLY, stat, mmap, PROT_READ, MAP_PRIVATE, MAP_FAILED, write, STDOUT_FILENO, size_t, ssize_t, fstat};

use rlpi::error_functions::{usage_err, err_exit, fatal};

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 || args[1] == "--help" {
        usage_err(&format!("{} file", args[0]));
    }

    let fd = unsafe {
        let path_s = CString::new(args[1].as_str()).expect("Failed to create CString");
        open(path_s.as_ptr(), O_RDONLY)
    };

    if fd == -1 {
        err_exit("open");
    }

    // obtain the size of the file and use it to specify the size of the mapping
    // and the size of the buffer to be written
    let sb: stat = unsafe {
        let mut sb: MaybeUninit<stat> = MaybeUninit::uninit();
        if fstat(fd, sb.as_mut_ptr()) == -1 {
            err_exit("fstat");
        }
        sb.assume_init()
    };

    let addr = unsafe { mmap(ptr::null_mut(), sb.st_size as size_t, PROT_READ, MAP_PRIVATE, fd, 0) };
    if addr == MAP_FAILED {
        err_exit("mmap");
    }

    let bytes_written = unsafe { write(STDOUT_FILENO, addr, sb.st_size as size_t) };
    if bytes_written != sb.st_size as ssize_t {
        fatal("partial/failed write");
    }
}