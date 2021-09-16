use std::mem::MaybeUninit;
use std::os::raw::{c_int};
use crate::libc::sys::sem::{semun};

use libc::{semctl, IPC_STAT, semid_ds};

pub fn stat(sem_id: c_int) -> Result<semid_ds, String> {
    unsafe {
        let mut ds: MaybeUninit<semid_ds> = MaybeUninit::uninit();
        let arg = semun { buf: ds.as_mut_ptr() };
        if semctl(sem_id, 0, IPC_STAT, arg) == -1 {
            Err("STAT failed".to_string())
        } else {
            Ok(ds.assume_init())
        }
    }
}