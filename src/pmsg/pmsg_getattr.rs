//listing 52-3 (page 1071)
use std::env;
use std::ffi::{CString};
use std::mem::MaybeUninit;

use libc::{mq_open, O_RDONLY, mq_attr, mq_getattr};

use rlpi::error_functions::{usage_err, err_exit};


pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 || args[1] == "--help" {
        usage_err(&format!("{} mq-name", args[0]));
    }

    let mqd = unsafe {
        let name_s = CString::new(args[1].as_str()).expect("Failed to create CString");
        mq_open(name_s.as_ptr(), O_RDONLY)
    };
    if mqd == -1 {
        err_exit("mq_open");
    }

    let attr = unsafe {
        let mut attr: MaybeUninit<mq_attr> = MaybeUninit::uninit();
        if mq_getattr(mqd, attr.as_mut_ptr()) == -1 {
            err_exit("mq_getattr");
        }
        attr.assume_init()
    };

    println!("Maximum number of messages on queue:   {}", attr.mq_maxmsg);
    println!("Maximum message size:                  {}", attr.mq_msgsize);
    println!("Number of messages currently on queue: {}", attr.mq_curmsgs);
}