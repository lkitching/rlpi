// listing 54-4 (page 1114)
use std::env;
use std::ffi::{CString};

use libc::{shm_unlink};

use rlpi::error_functions::{usage_err, err_exit};

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 || args[1] == "--help" {
        usage_err(&format!("{} shm-name", args[0]))
    }

    unsafe {
        let name_s = CString::new(args[1].as_str()).expect("Failed to create CString");
        if shm_unlink(name_s.as_ptr()) == -1 {
            err_exit("shm_unlink");
        }
    }
}