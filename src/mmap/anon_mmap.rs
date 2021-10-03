// listing 49-3 (page 1036)
use std::{ptr, mem, env};
use std::os::raw::{c_void};
use std::ffi::{CString};

use libc::{mmap, PROT_READ, PROT_WRITE, MAP_SHARED, MAP_ANONYMOUS, MAP_FAILED, open, O_RDWR, close,
           munmap, exit, EXIT_SUCCESS, wait};
use rlpi::error_functions::err_exit;
use rlpi::util::{fork_or_die, ForkResult};


fn map_anon() -> Result<*mut u32, String> {
    unsafe {
        let addr = mmap(
            ptr::null_mut(),
            mem::size_of::<u32>(),
            PROT_READ | PROT_WRITE,
            MAP_SHARED | MAP_ANONYMOUS,
            -1,
            0);
        if addr == MAP_FAILED {
            Err("mmap failed".to_string())
        } else {
            Ok(addr as *mut u32)
        }
    }
}

fn map_dev_zero() -> Result<*mut u32, String> {
    unsafe {
        let cs = CString::new("/dev/zero").expect("Failed to create CString");
        let fd = open(cs.as_ptr(), O_RDWR);
        if fd == -1 {
            return Err("failed to open /dev/zero file descriptor".to_string());
        }

        let addr = mmap(
            ptr::null_mut(),
            mem::size_of::<u32>(),
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            fd,
            0);

        if addr == MAP_FAILED {
            return Err("mmap failed".to_string());
        }

        if close(fd) == -1 {
            return Err("failed to close /dev/zero file descriptor".to_string());
        }

        Ok(addr as *mut u32)
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    // map using /dev/zero if first argument is 'z'
    let use_dev_zero = args.len() > 1 && args[1] == "z";

    let map_result = if use_dev_zero { map_dev_zero() } else { map_anon() };
    match map_result {
        Err(msg) => {
            err_exit(&format!("Failed to create mapping: {}", msg))
        },
        Ok(addr) => {
            // initialise integer in mapped region
            unsafe { *addr = 1; }

            match fork_or_die() {
                ForkResult::Parent(_child_pid) => {
                    // wait for child to terminate
                    if unsafe { wait(ptr::null_mut()) } == -1 {
                        err_exit("wait");
                    }
                    println!("In parent: {}", unsafe { *addr });

                    if unsafe { munmap(addr as *mut c_void, mem::size_of::<u32>()) } == -1 {
                        err_exit("munmap");
                    }
                },
                ForkResult::Child => {
                    println!("Child started, value = {}", unsafe { *addr });
                    unsafe { *addr += 1; }

                    if unsafe { munmap(addr as *mut c_void, mem::size_of::<u32>()) } == -1 {
                        err_exit("munmap");
                    }
                    unsafe { exit(EXIT_SUCCESS); }
                }
            }
        }
    }
}