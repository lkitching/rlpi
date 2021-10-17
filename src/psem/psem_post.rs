// listing 53-4 (page 1097)
use std::env;

use libc::{sem_open, sem_post};
use rlpi::error_functions::{usage_err, err_exit};
use rlpi::psem::sem_utils::open_existing;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 || args[1] == "--help" {
        usage_err(&format!("{} sem-name", args[0]));
    }

    let sem = open_existing(args[1].as_str()).unwrap();
    if unsafe { sem_post(sem) } == -1 {
        err_exit("sem_post");
    }
}