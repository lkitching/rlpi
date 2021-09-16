// listing 47-4 (page 974)
use std::env;
use std::os::raw::c_ushort;
use rlpi::error_functions::{usage_err, cmd_line_err, err_exit};

use rlpi::svsem::sem_util;
use rlpi::libc::sys::sem::{semun, SETALL};

use libc::{getpid, semctl};

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args[1] == "--help" {
        usage_err(&format!("{} semid val ...", args[0]));
    }

    let sem_id = args[1].parse().expect("Invalid semaphore id");
    let ds = sem_util::stat(sem_id).expect("Failed to get semaphore");

    let mut sem_values: Vec<c_ushort> = args[2 ..].iter().map(|s| s.parse().expect("Invalid semaphore value")).collect();

    if sem_values.len() != ds.sem_nsems as usize {
        cmd_line_err(&format!("Set contains {} semaphores but {} values were supplied\n",
                              ds.sem_nsems,
                              sem_values.len()));
    }

    let arg = semun { array: sem_values.as_mut_ptr() };
    if unsafe { semctl(sem_id, 0, SETALL, arg) } == -1 {
        err_exit("semctl SETALL");
    }
    println!("Semaphore values changed (PID={})", unsafe { getpid() });
}