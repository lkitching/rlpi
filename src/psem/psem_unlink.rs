// listing 53-2 (page 1094)
use std::{env};
use std::ffi::CString;

use libc::{sem_unlink};

use rlpi::error_functions::{usage_err, err_exit};

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 || args[1] == "--help" {
        usage_err(&format!("{} sem-name", args[0]));
    }

    let name_s = CString::new(args[1].as_str()).expect("Failed to create CString");
    if unsafe { sem_unlink(name_s.as_ptr()) } == -1 {
        err_exit("sem_unlink");
    }
}