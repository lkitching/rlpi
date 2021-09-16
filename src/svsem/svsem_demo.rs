// listing 47-1 (page 968)
use std::env;
use std::os::raw::{c_int, c_ushort};

use libc::{semget, IPC_PRIVATE, S_IRUSR, S_IWUSR, semctl, semid_ds, seminfo, sembuf, getpid, semop};

use rlpi::error_functions::{usage_err, err_exit};
use rlpi::curr_time::curr_time;
use rlpi::libc::sys::sem::{semun, SETVAL};

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 || args[1] == "--help" {
        usage_err(&format!("{} init-value\n\tor: {} semid operation", args[0], args[0]));
    }

    if let Some(op_str) = args.get(2) {
        let sem_id = args[1].parse().expect("Invalid semaphore id");
        let op_id = op_str.parse().expect("Invalid operation");

        let mut sop = sembuf {
            sem_num: 0,     // update first semaphore in the set
            sem_op: op_id,
            sem_flg: 0      // no flags
        };

        let pid = unsafe { getpid() };
        println!("{}: about to semop at {}", pid, curr_time("%T"));
        if unsafe { semop(sem_id, &mut sop, 1) } == -1 {
            err_exit("semop");
        }

        println!("{}: semop completed at {}", pid, curr_time("%T"));
    } else {
        // create and initialise semaphore
        let flags = (S_IRUSR | S_IWUSR) as c_int;
        let sem_id = unsafe { semget(IPC_PRIVATE, 1, flags) };
        if sem_id == -1 {
            err_exit("semget");
        }

        let initial_value = args[1].parse().expect("Invalid initial value");
        let val = semun { val: initial_value };
        if unsafe { semctl(sem_id, 0, SETVAL, val) } == -1 {
            err_exit("semctl");
        }

        println!("Semaphore ID = {}", sem_id);
    }
}