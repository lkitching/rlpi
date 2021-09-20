//listing 20-3 (page 405)
use std::env;

use libc::{exit, EXIT_SUCCESS, kill, EPERM, ESRCH};

use rlpi::libc::{errno};
use rlpi::error_functions::{usage_err, err_exit};

pub fn main() {
	let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
		usage_err(&format!("{} signum pid", args[0]));
    }

    let pid = args[2].parse().expect("Invalid process ID");
    let sig = args[1].parse().expect("Invalid signal ID");

    let s = unsafe { kill(pid, sig) };

    if sig != 0 {
		if s == -1 {
			err_exit("kill");
		}
	} else {
		if s == 0 {
			println!("Process {} exists and we can send it signals", pid);
		} else {
			let e = errno();
			if e == EPERM {
				println!("Process {} exists but we don't have permission to send signals", pid);
			}
			else if e == ESRCH  {
				println!("Process {} does not exist", pid);
			}
			else {
				err_exit("kill");
			}
		}
    }

    unsafe { exit(EXIT_SUCCESS); }
}
