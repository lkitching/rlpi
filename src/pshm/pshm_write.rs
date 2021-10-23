// listing 54-2 (page 1112)
use std::env;
use std::ptr;
use std::ffi::{CString};
use std::os::raw::{c_void};

use libc::{O_RDWR, ftruncate, off_t, PROT_READ, PROT_WRITE, MAP_SHARED, MAP_FAILED, close, memcpy, shm_open, mmap};

use rlpi::error_functions::{usage_err, err_exit};

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        usage_err(&format!("{} shm-name string", args[0]));
    }

    // open existing object
    let fd = unsafe {
        let name_s = CString::new(args[1].as_str()).expect("Failed to create CString");
        shm_open(name_s.as_ptr(), O_RDWR, 0)
    };

    if fd == -1 {
        err_exit("shm_open");
    }

    // resize object to hold string
    let msg = args[2].as_str();
    let len = msg.bytes().len();

    if unsafe { ftruncate(fd, len as off_t) } == -1 {
        err_exit("ftruncate");
    }

    let addr = unsafe { mmap(ptr::null_mut(), len, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0) };
    if addr == MAP_FAILED {
        err_exit("mmap");
    }

    // fd no longer needed
    if unsafe { close(fd) } == -1 {
        err_exit("close");
    }

    println!("copying {} bytes", len);
    unsafe { memcpy(addr, msg.as_ptr() as *const c_void, len); }
}