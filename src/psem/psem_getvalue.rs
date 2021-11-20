// listing 53-5 (page 1098)
use std::env;

use libc::{sem_getvalue};

use rlpi::psem::sem_utils::open_existing;
use rlpi::error_functions::{usage_err, err_exit};

pub fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 || args[1] == "--help" {
        usage_err(&format!("{} sem-name", args[0]));
    }

    let sem = open_existing(args[1].as_str()).unwrap();
    
    let mut value = 0;
    if unsafe { sem_getvalue(sem, &mut value) } == -1 {
        err_exit("sem_getvalue");
    }

    println!("{}", value);
}