// listing 47-3 (page 973)
use std::env;
use rlpi::error_functions::{usage_err, err_exit};
use std::mem::MaybeUninit;
use std::os::raw::{c_ushort, c_int};

use libc::{semid_ds, semctl, IPC_STAT};

use rlpi::libc::sys::sem::{semun, GETALL, GETPID, GETNCNT, GETZCNT};
use rlpi::libc::time::ctime_string;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 || args[1] == "--help" {
        usage_err(&format!("{} semid", args[0]));
    }

    let sem_id = args[1].parse().expect("Invalid semaphore id");

    let ds = unsafe {
        let mut ds: MaybeUninit<semid_ds> = MaybeUninit::uninit();
        let arg = semun { buf: ds.as_mut_ptr() };
        if semctl(sem_id, 0, IPC_STAT, arg) == -1 {
            err_exit("semctl");
        }
        ds.assume_init()
    };

    println!("Semaphore changed: {}", ctime_string(&ds.sem_ctime));
    println!("Last semop(): {}", ctime_string(&ds.sem_otime));

    // display per-semaphore information
    let values = {
        let mut sems: Vec<c_ushort> = Vec::with_capacity(ds.sem_nsems as usize);
        let arg = semun { array: sems.as_mut_ptr() };
        if unsafe { semctl(sem_id, 0, GETALL, arg) } == -1 {
            err_exit("semtcl GETALL");
        }
        unsafe { sems.set_len(ds.sem_nsems as usize); }
        sems
    };

    println!("Sem #   Value   SEMPID  SEMNCNT  SEMZCNT");
    for j in 0 .. values.len() {
        println!("{}   {}   {}  {}    {}",
                 j,
                 values[j],
                 unsafe { semctl(sem_id, j as c_int, GETPID) },
                 unsafe { semctl(sem_id, j as c_int, GETNCNT) },
                 unsafe { semctl(sem_id, j as c_int, GETZCNT) });
    }
}