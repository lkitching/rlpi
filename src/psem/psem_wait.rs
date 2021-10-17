// listing 53-3 (page 1095)
use std::env;
use std::ffi::CString;

use libc::{sem_open, SEM_FAILED, sem_wait, getpid};

use rlpi::error_functions::{usage_err, err_exit};

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "--help" {
        usage_err(&format!("{} sem-name", args[0]));
    }

    let sem = unsafe {
        let name_s = CString::new(args[1].as_str()).expect("Failed to create CString");
        sem_open(name_s.as_ptr(), 0)
    };
    if sem == SEM_FAILED {
        err_exit("sem_wait");
    }

    if unsafe { sem_wait(sem) } == -1 {
        err_exit("sem_wait");
    }

    println!("{} sem_wait() succeeded", unsafe { getpid() });
}