// listing 54-3 (page 1113)
use std::env;
use std::ffi::{CString};
use std::ptr;

use libc::{shm_open, O_RDWR, stat, fstat, mmap, close, STDOUT_FILENO, write, PROT_READ, MAP_SHARED, MAP_FAILED, size_t};

use rlpi::error_functions::{usage_err, err_exit};
use std::mem::MaybeUninit;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 || args[1] == "--help" {
        usage_err(&format!("{} shm-name", args[0]));
    }

    let fd = unsafe {
        let name_s = CString::new(args[1].as_str()).expect("Failed to create CString");
        shm_open(name_s.as_ptr(), O_RDWR, 0)
    };

    if fd == -1 {
        err_exit("shm_open");
    }

    let sb = unsafe {
        let mut sb: MaybeUninit<stat> = MaybeUninit::uninit();
        if fstat(fd, sb.as_mut_ptr()) == -1 {
            err_exit("fstat");
        }
        sb.assume_init()
    };

    let addr = unsafe { mmap(ptr::null_mut(), sb.st_size as size_t, PROT_READ, MAP_SHARED, fd, 0) };
    if addr == MAP_FAILED {
        err_exit("mmap");
    }

    // fd is no longer needed
    if unsafe { close(fd) } == -1 {
        err_exit("close");
    }

    // write contents of shared memory to stdout
    unsafe { write(STDOUT_FILENO, addr, sb.st_size as size_t); }
    println!();
}
