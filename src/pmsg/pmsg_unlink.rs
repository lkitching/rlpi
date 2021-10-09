//listing 52-1 (page 1066)
use std::env;
use std::ffi::{CString};

use libc::{mq_unlink};

use rlpi::error_functions::{usage_err, err_exit};

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 || args[1] == "--help" {
        usage_err(&format!("{} mq-name", args[0]));
    }

    unsafe {
        let queue_name_s = CString::new(args[1].as_str()).expect("Failed to create CString");
        if mq_unlink(queue_name_s.as_ptr()) == -1 {
            err_exit("mq_unlink");
        }
    }
}