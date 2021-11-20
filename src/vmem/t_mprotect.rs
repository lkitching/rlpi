// listing 50-1 (page 1046)
use std::ptr;
use std::ffi::{CString};

use libc::{mmap, PROT_NONE, MAP_SHARED, MAP_ANONYMOUS, MAP_FAILED, getpid, system, PROT_READ, PROT_WRITE, mprotect};
use rlpi::error_functions::err_exit;

const LEN: usize = 1024 * 1024;

fn display_maps() {
    let cmd = format!("cat /proc/{}/maps | grep zero", unsafe { getpid() });
    let cmd_s = CString::new(cmd).expect("Failed to create CString");
    let _res = unsafe { system(cmd_s.as_ptr()) };
}

pub fn main() {
    // create an anonymous mapping with all access denied
    let addr = unsafe { mmap(ptr::null_mut(), LEN, PROT_NONE, MAP_SHARED | MAP_ANONYMOUS, -1, 0) };
    if addr == MAP_FAILED {
        err_exit("mmap");
    }

    // display line from /proc/self/maps corresponding to mapping
    println!("Before mprotect()");
    display_maps();

    // change memory protected to allow read/write access
    unsafe {
        if mprotect(addr, LEN, PROT_READ | PROT_WRITE) == -1 {
            err_exit("mprotect");
        }
    }

    println!("After mprotect()");
    display_maps();
}