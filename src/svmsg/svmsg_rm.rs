//listing 46-4 (page 947)
use std::env;
use std::ptr;
use std::os::raw::{c_int};

use libc::{msgctl, IPC_RMID};

extern crate rlpi;
use rlpi::error_functions::{usage_err, err_exit};

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.get(0).map(|s| s.as_str()) == Some("--help") {
        usage_err(&format!("{} [msqid ...]", args[0]));
    }

    let ids: Vec<c_int> = args[1..].iter().map(|s| s.parse().expect("Invalid message queue id")).collect();
    for id in ids.into_iter() {
        if unsafe { msgctl(id, IPC_RMID, ptr::null_mut()) } == -1 {
            err_exit(&format!("msgctl {}", id));
        }
    }
}