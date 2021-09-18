//listing 46-5 (page 949)
use std::env;

use libc::{msqid_ds, msgctl, IPC_STAT, IPC_SET};
use std::mem::MaybeUninit;
use rlpi::error_functions::{usage_err, err_exit};

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 || args[1] == "--help" {
        usage_err(&format!("{} msqid max-bytes", args[0]));
    }

    let msqid = args[1].parse().expect("Invalid message queue id");
    let max_bytes = args[2].parse().expect("Invalid max bytes");

    let mut ds = unsafe {
        let mut ds: MaybeUninit<msqid_ds> = MaybeUninit::uninit();
        if msgctl(msqid, IPC_STAT, ds.as_mut_ptr()) == -1 {
            err_exit("msgctl");
        }
        ds.assume_init()
    };

    ds.msg_qbytes = max_bytes;

    if unsafe { msgctl(msqid, IPC_SET, &mut ds) } == -1 {
        err_exit("msgctl");
    }
}