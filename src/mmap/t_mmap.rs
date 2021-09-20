// listing 49-2 (page 1028)
use std::{env, ptr};
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_void};

use libc::{open, O_RDWR, size_t, mmap, PROT_READ, PROT_WRITE, MAP_SHARED, MAP_FAILED, close, memset, msync, MS_SYNC, memcpy};
use rlpi::error_functions::{usage_err, err_exit, cmd_line_err};

const MEM_SIZE: size_t = 10;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "--help" {
        usage_err(&format!("{} file [new-value]", args[0]));
    }

    let fd = unsafe {
        let cs = CString::new(args[1].as_str()).expect("Failed to create CString");
        open(cs.as_ptr(), O_RDWR)
    };

    if fd == -1 {
        err_exit("open");
    }

    let addr = unsafe { mmap(ptr::null_mut(), MEM_SIZE, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0) };
    if addr == MAP_FAILED {
        err_exit("mmap");
    }

    // no longer need file descriptor
    if unsafe { close(fd) } == -1 {
        err_exit("close");
    }

    {
        let cs = unsafe { CStr::from_ptr(addr as *const c_char) };
        println!("Current string={:10}", cs.to_str().expect("Failed to convert"));
    }

    if args.len() > 2 {
        // update contents of region
        let bytes = args[2].as_bytes();
        if bytes.len() >= MEM_SIZE {
            cmd_line_err("'new-value' is too large");
        }

        // zero out region
        unsafe { memset(addr, 0, MEM_SIZE); }

        unsafe { memcpy(addr, bytes.as_ptr() as *const c_void, bytes.len()); }

        if unsafe { msync(addr, MEM_SIZE, MS_SYNC) } == -1 {
            err_exit("msync");
        }

        println!("Copied {} to shared memory", args[2]);
    }
}