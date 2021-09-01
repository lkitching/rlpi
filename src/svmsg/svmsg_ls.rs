// listing 46-6 (page 953)
use std::os::raw::{c_int};
use std::mem::{MaybeUninit};

use libc::{msqid_ds, msginfo, msgctl, MSG_INFO, MSG_STAT, EINVAL, EACCES};

use rlpi::error_functions::{err_exit, err_msg};
use rlpi::libc::errno;

enum QueueLookupResult {
    NotFound,
    Found { msqid: c_int, ds: msqid_ds }
}

fn get_queue_by_index(index: c_int) -> Result<QueueLookupResult, String> {
    unsafe {
        let mut ds: MaybeUninit<msqid_ds> = MaybeUninit::uninit();
        let msqid = msgctl(index, MSG_STAT, ds.as_mut_ptr());
        if msqid == -1 {
            let e = errno();
            if e != EINVAL && e != EACCES {
                Err("msgctl-MSG_STAT".to_owned())
            } else {
                Ok(QueueLookupResult::NotFound)
            }
        } else {
            Ok(QueueLookupResult::Found{ ds: ds.assume_init(), msqid: msqid })
        }
    }
}

pub fn main() {
    let max_index = unsafe {
        let mut msg_info: MaybeUninit<msginfo> = MaybeUninit::uninit();
        msgctl(0, MSG_INFO, msg_info.as_mut_ptr() as *mut msqid_ds)
    };

    if max_index == -1 {
        err_exit("msgtcl-MSG_INFO");
    }

    println!("maxind: {}", max_index);
    println!();
    println!("index    id      key    messages");

    // retrieve and display information from each element of the entries array
    for index in 0 .. max_index + 1 {
        match get_queue_by_index(index) {
            Err(msg) => {
                err_msg(&msg);
            },
            Ok(QueueLookupResult::NotFound) => {
                // ignore this item
            },
            Ok(QueueLookupResult::Found { ds, msqid }) => {
                println!("{} {}  {:#010x} {}",
                    index,
                    msqid,
                    ds.msg_perm.__key,
                    ds.msg_qnum
                );
            }
        }
    }
}